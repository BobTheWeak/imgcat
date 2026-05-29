
import type { PageServerLoad, Actions } from './$types';
import { redirect, fail, json } from '@sveltejs/kit';
import { jwtDecode, jwtVerify } from '$lib/server/jwt2';
import { IsUsernameFree } from '$lib/is_username_free.remote.ts';


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
	}
}
