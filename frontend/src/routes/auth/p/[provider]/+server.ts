import type { RequestHandler } from './$types';

import { redirect, error } from '@sveltejs/kit';

const KNOWN_PROVIDERS = ['google', 'microsoft'];

export const GET: RequestHandler = async ({ locals, params, url, request }) => {
	if(locals.logged_in) {redirect(307, '/')}

	if(process.env.IC_FRONTEND_PVT_SVR) {
		// Check the cookie, and see if it contains this private server's code
		const pvt_auth = cookies.get('ic_private');
		if(pvt_auth) {
			// Support a semicolon-separated list of values, to support multiple servers
			if(!pvt_auth.value.split(";").includes(process.env.IC_FRONTEND_PVT_SVR)) {
				error(403, 'Private server');
			}
		} else {
			error(401, 'Private server');
		}
	}

	if(KNOWN_PROVIDERS.includes(params.provider)) {
		//console.log(params.provider);

		//let api_url = new URL(process.env.IC_LOC_INT);
		//console.log(url);
		//if(api_url.protocol) { url.protocol = api_url.protocol }
		//if(api_url.host) { url.host = api_url.host }
		//if(api_url.port) { url.port = api_url.port }
		//if(api_url.pathname) { url.pathname = api_url.pathname + '/auth/p/' + params.provider }
		//console.log(url);
		let api_url = new URL(process.env.IC_LOC_INT);
		api_url.pathname += '/auth/p/' + params.provider

		const res = await fetch(api_url);
		//console.log(res);

		if(res.status === 200) {
			redirect(307, res.url);
		} else {
			error(500);
		}
	} else {
		error(404);
	}
}