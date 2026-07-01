import type { RequestHandler } from './$types';

import { redirect, error } from '@sveltejs/kit';

const KNOWN_PROVIDERS = ['google'];

export const GET: RequestHandler = async ({ locals, params }) => {
	if(locals.logged_in) {redirect(307, '/')}

	if(KNOWN_PROVIDERS.includes(params.provider)) {
		const res = await fetch(process.env.IC_LOC_INT + '/auth/' + params.provider);
		if(res.status === 200) {
			redirect(307, res.url);
		} else {
			error(500);
		}
	} else {
		error(404);
	}
}