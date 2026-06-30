import type { ServerPageLoad, Actions } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: ServerPageLoad = async({ locals, cookies, fetch }) => {
	if(!locals.logged_in){redirect(307, '/login')}

	const ajwt = cookies.get('ic_auth');
	if(!ajwt){redirect(307, '/login')}

	const res = await fetch(process.env.IC_LOC_INT+'/auth/my/prefs', {
		headers: {
			"Authorization": 'Bearer ' + ajwt
		}
	});

	const data = await res.json();
	
	return {
		prefs: data
	};
}

export const actions = {
	save: async({ locals, cookies, fetch, request }) => {
		if(!locals.logged_in){redirect(307, '/login')}

		const ajwt = cookies.get('ic_auth');
		if(!ajwt){redirect(307, '/login')}

		// Convert from a form post to form-encoded
		const form_data = new URLSearchParams(await request.formData())
		form_data.append('account_id', locals.user_id);

		const res = await fetch(process.env.IC_LOC_INT+'/auth/my/prefs', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded',
				'Authorization': 'Bearer ' + ajwt
			},
			body: new URLSearchParams(form_data)
		});

		if(res.status === 200) {
			console.log("success");
			const num = await res.text();
			return {success: true, updated: Number.parseInt(num)}
		} else {
			console.log("failure");
			console.log(res);
			console.log(await res.text());
			return {success: false}
		}
	},
	logout: async({ cookies }) => {
		cookies.delete('ic_auth', {path:'/'});
		cookies.delete('ic_refresh', {path:'/'});
		redirect(303, '/home');
	}
}