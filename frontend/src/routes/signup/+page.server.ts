
import type { PageServerLoad, Actions } from './$types';
import { redirect, fail, json } from '@sveltejs/kit';
import { jwtDecode, jwtVerify } from '$lib/server/jwt2';
import { IsUsernameFree } from '$lib/is_username_free.remote.ts';
import { parseCookie } from 'cookie';


// Borrowed/simplified from hooks.server.ts
// TODO: Shared library? (prolly not worth the effort)
function cookie_to_svelte_opts(cookie_obj) {
	const result = {};
	if(cookie_obj['Path']) {result['path'] = cookie_obj['Path']}
	if(cookie_obj['Max-Age']) {result['maxAge'] = cookie_obj['Max-Age']}
	if(cookie_obj['SameSite']) {result['sameSite'] = cookie_obj['SameSite']}
	return result;
}


export const load: PageServerLoad = async({ locals, cookies, request }) => {

	// If they're already logged in, redirect to home
	// TODO: We should be smarter, and check if the JWT is valid, etc, etc.
	let rjwt = cookies.get("ic_refresh");
	if(rjwt) { return redirect(307, '/home') }

	// Verify the signup JWT exists
	let sjwt = cookies.get("ic_signup");
	if(!sjwt) { return redirect(307, '/login?error_msg=Please+sign+in+first') }
	sjwt = jwtDecode(sjwt);
	if(!sjwt) { return redirect(307, '/login?error_msg=Please+sign+in+first') }

	return {
		prv: sjwt.prv,
		sub: sjwt.sub,
		cc: sjwt.cc,
		age: sjwt.age,
	};
}


export const actions:Actions = {
	default: async({ cookies, request, fetch }) => {

		// Verify the signup JWT exists
		const jwt_string = cookies.get("ic_signup");
		if(!jwt_string) { return redirect(307, '/login?error_msg=Please+sign+in+first') }
		const jwt = jwtDecode(jwt_string);
		if(!jwt) { return redirect(307, '/login?error_msg=Please+sign+in+first') }

		const fd = await request.formData();
		const username = fd.get('username');

		// Check if TOC is checked (it's required, so it should always be)
		if(fd.get('toc') !== 'on'){
			return {success:false, message:"Please accept the terms of use", username: username};
		}

		// Check the username again
		const namefree = await IsUsernameFree(username);
		if(!namefree) {
			return {success:false, message:"Username is not available", username: username};
		}

		// Creating a new FormData to keep it clean (in case shenanigans)
		const post_data = new URLSearchParams();
		post_data.append('prv', jwt.prv);
		post_data.append('sub', jwt.sub);
		post_data.append('user', username);

		const signup_response = await fetch('/api/auth/create', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded',
				'Authorization': 'Bearer ' + jwt_string
			},
			body: post_data.toString()
		});

		console.log(signup_response);

		if(signup_response.status === 201) {

			// Grab the cookies returned, and set them
			for(const c_str of signup_response.headers.getSetCookie()) {
				const c_name = c_str.substring(0, c_str.indexOf("="));
				if(c_name?.startsWith("ic_")) {
					const c_obj = parseCookie(c_str);
					const c_val = c_obj[c_name];
					console.log(c_name, c_val);
					console.log(c_obj);
					cookies.set(c_name, c_val, cookie_to_svelte_opts(c_obj));
				}
			}

			// Cleanup the signup cookie
			cookies.delete("ic_signup", {path:"/"});

			// Success!
			redirect(307, '/home');
		}

		return {success:false, message:"There was a problem creating your account", username: username}
	}
}



/*
for(const cookie_str of refresh_response.headers.getSetCookie()) {
	const cookie_obj = parseCookie(cookie_str);
	if(cookie_obj) {
		const cookie_val = cookie_obj['ic_auth'];
		const cookie_opt = cookie_to_svelte_opts(cookie_obj);

	}
	event.cookies.set('ic_auth', cookie_val, cookie_opt);

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
*/