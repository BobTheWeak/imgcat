import type { RequestHandler } from './$types';

import { redirect, error } from '@sveltejs/kit';
import { cookie_to_svelte_opts } from '$lib/server/cookie.ts';
import { parseCookie } from 'cookie';

const KNOWN_PROVIDERS = ['google', 'microsoft'];


export const GET: RequestHandler = async ({ locals, params, url, request, cookies }) => {
	if(locals.logged_in) {redirect(307, '/')}

	if(KNOWN_PROVIDERS.includes(params.provider)) {
		let api_url = new URL(process.env.IC_LOC_INT);
		api_url.pathname += '/auth/cb/' + params.provider
		api_url.search = url.search;

		const res = await fetch(api_url);

		if(res.status === 200) {
			const redirect_url = res.headers.get("Location") || '/home';

			for(const c_str of res.headers.getSetCookie()) {
				const c_name = c_str.substring(0, c_str.indexOf("="));
				if(c_name?.startsWith("ic_")) {
					const c_obj = parseCookie(c_str);
					const c_val = c_obj[c_name];
					cookies.set(c_name, c_val, cookie_to_svelte_opts(c_obj));
				}
			}

			redirect(307, redirect_url);
		} else {
			error(500);
		}
	} else {
		error(404);
	}
}