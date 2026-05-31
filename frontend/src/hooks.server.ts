import type { Handle, HandleFetch } from '@sveltejs/kit';
import { jwtDecode, jwtValidate } from '$lib/server/jwt2.ts';
import { parseCookie } from 'cookie';

import { building } from '$app/environment'

// The cookie library decodes a cookie into different field names than
// Svelte's cookie.set(name, val, opts) accepts, so we have to translate
// NOTE: There's a similar version copied & pasted for /signup (server-side)
function cookie_to_svelte_opts(cookie_obj) {
	const translations = {
		'Path': 'path',
		'Max-Age': 'maxAge',
		'SameSite': 'sameSite'
		// TODO: These are unverified, but theoretically accurate
		// 'Domain': 'domain',
		// 'Expires': 'expires',
		// 'Secure': 'secure',
		// 'HttpOnly': 'httpOnly',
		// 'Partitioned': 'partitioned'
	}
	const result = {};
	for(const k in cookie_obj) {
		if(translations[k]) {
			result[translations[k]] = cookie_obj[k];
		}
	}
	return result;
}


export const handle: Handle = async ({ event, resolve }) => {

	// Assume stranger-danger until proven otherwise
	event.locals.logged_in = false;
	// TODO: We want to move away from this being browser-side,
	// but for now, web stuff depends on it...
	event.locals.content_level = 2;

	// Grab the auth JWT
	// This is the main, short-lived (~5 min) JWT with data storage
	const auth_s = event.cookies.get('ic_auth');
	const auth_j = jwtDecode(auth_s);
	if(auth_s && !auth_j) {
		event.cookies.delete('ic_auth', {path:'/'});
	}

	if(auth_j) {
		// Happy path - just load the cookie data into locals so pages can use it
		// NOTE: Keep aligned with code in the more complicated path below - TODO: simplify
		event.locals.logged_in = true;
		event.locals.user_id = auth_j.sub;
		event.locals.username = auth_j.user;
		event.locals.claims = auth_j.claims;
	} else {
		// Ok, slightly more complicated path, check if we can refresh the auth token

		// Grab the refresh JWT
		// This is the longer-lived (~2 wks) JWT that we use for manual signins
		const refresh_s = event.cookies.get('ic_refresh');
		const refresh_valid = jwtValidate(refresh_s);
		if(refresh_s && !refresh_valid) {
			event.cookies.delete('ic_refresh', {path:'/'});
			event.cookies.delete('ic_auth', {path:'/'});
		}

		if(refresh_valid) {
			// Still good, so grab a new one from the Auth service
			const refresh_response = await event.fetch('/api/auth/refresh', {
				method: 'GET',
				headers: {'Authorization': 'Bearer ' + refresh_s}
			});

			// It returns a body-less response, with JWTs in the Set-Cookie header
			// Parsing it out kinda sucks, but that's OK
			if(refresh_response.status === 200) {
				for(const cookie_str of refresh_response.headers.getSetCookie()) {
					const cookie_obj = parseCookie(cookie_str);
					const cookie_val = cookie_obj['ic_auth'];
					if(cookie_val){
						// Extract the core data into locals
						const ajwt = jwtDecode(cookie_val);
						const cookie_opt = cookie_to_svelte_opts(cookie_obj);
						if(ajwt) {
							// Set the cookie
							event.cookies.set('ic_auth', cookie_val, cookie_opt);

							// NOTE: Keep aligned with code in the happy-path above - TODO: simplify
							event.locals.logged_in = true;
							event.locals.user_id = ajwt.sub;
							event.locals.username = ajwt.user;
							event.locals.claims = ajwt.claims;
						}
					}
				}
			} else if ([401, 403, 418, 400].includes(refresh_response.status)) {
				// 401 - Missing Auth Header
				// 403 - Header validation issues
				// 418 - User is banned (IDK if temp or permanently)
				// 400 - Could not find that user in the DB
				// In these cases, delete the refresh cookie
				event.cookies.delete('ic_refresh', {path:'/'});
			} else {
				// Service outage. Maybe they can refresh?
			}
		}
	}

	return resolve(event);
}


// Svelte rewrites fetch() to do fancy, in-app things. One of those is intercepting
// all HTTP fetch() calls and rerouting them to Svelte pages w/o the HTTP overhead.
// But we can't do this... anything /api MUST be routed through the proxy, not Svelte.
// Mechanically, we get a Request w/ modifications, and we clobber it with a base one.
export const handleFetch:HandleFetch = async({ request, fetch }) => {
	const url = new URL(request.url);
	if (url.pathname.startsWith('/api')) {

		// It gets a little weird behind a proxy
		url.hostname = new URL(process.env.IC_ORIGIN).hostname;

		// Clone the original request so Svelte doesn't serve it internally
		request = new Request(url, request);
	}

	return fetch(request);
}