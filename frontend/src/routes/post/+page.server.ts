import { fileTypeFromBlob } from 'file-type';

import type { ServerPageLoad, Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';

import { CreatePostAndUpload } from '$lib/server/create_post.ts';

///// Variables /////
const MAX_FILE_SIZE:int = 31457280; // 30MiB
const ALLOWED_MIME_TYPES = [
	// Raster images
	'image/png', 'image/jpeg', 'image/webp',
	// Vector images
	'image/svg',
	// Animations
	'image/gif', 'image/apng',
	// Videos
	// TODO: Not implemented in first release
	'video/mp4','video/webm'
];
/////////////////////



export const load: ServerPageLoad = async({ locals }) => {
	if(!locals.logged_in){redirect(307, '/login')}
	return locals;
}

export const actions:Actions = {
	upload: async({ locals, request, fetch, clientAddress }) => {
		console.log("Upload Success");
		let form_data = await request.formData();
		let file = form_data.get('upload-file');
		if(file && file.size > 0) {
			console.log(file);

			if(file.size > 31457280){
				return fail(400, {err_msg:'File is too large'});
			}
			let verified_type = await fileTypeFromBlob(file);
			if(!ALLOWED_MIME_TYPES.includes(verified_type.mime)) {
				console.log('FAILED type');
				console.log(verified_type);
				return fail(400, {err_msg:'Unsupported file type'});
			}

			let id = await CreatePostAndUpload(locals.user_id, file, verified_type);
			if(id) {
				redirect(307, '/view/' + id);
				//return {success:true, post_id:id}
			} else {
				return fail(400, {err_msg: 'There was an unknown problem uploading the file'})
			}
		}
		return fail(400, {err_msg: 'There was an unknown problem uploading the file'});
	}//,
	// Not used at the moment... But this will create a new post & attach an existing
	// Media, which is super-efficient
	/*repost: async({ locals, request, fetch, clientAddress }) => {
		console.log("=== Repost ===");
		// Check the number of bytes sent
		console.log("locals");
		console.log(locals);
		console.log("request");
		console.log(request);
		console.log("fetch");
		console.log(fetch);
		console.log("clientAddress");
		console.log(clientAddress);

		let form_data = await request.formData();
		console.log("formData");
		console.log(form_data);
	}*/
} satisfies Actions;

/*
export async function POST({ request, clientAddress }) {
	// NOTE: Earlier versions would return errors as: error(403, "Validation error"), which is more streamlined
	//       But it returned: {status:403, message:'Forbidden', body:{message:'Validation Error'},
	//       which required a two-step promise handler, and was FAR more complicated on the client-side
	//       We're now always returning 200, with a success flag

	// TODO: These are printing to the console/stdout, but figure out how to log to a dedicated file

	const dt = new Date().toISOString();
	const data = await request.json();
	
	const email = data['e']?.toLowerCase();
	const password = data['p'];
	if(!email || !password) {
		console.log(dt+', Login Failure, POST data in an unexpected format');
		if(data['p']){data['p']='***HIDDEN***'}
		console.log(data);
		// NOTE: Slightly different user-error on purpose
		// This should never occur unless someone's doing shenanigans
		return json({success:false, message:"Missing data"});
	}

	// TODO: Thist should be stored as request.remoteAddress,
	// but it needs env vars to set headers as expected:
	// https://svelte.dev/docs/kit/adapter-node#Environment-variables-ADDRESS_HEADER-and-XFF_DEPTH
	//let client_ip = clientAddress || '::ffff:127.0.0.1';

	const user_data = await LogIn(email, password);
	if(user_data === null){
		console.log(dt+', Login Failure, Incorrect password, '+email);
		return json({success:false, message:"Incorrect password"})
	}

	// Now that we've got user_data, we need to sign a JWT and return it
	console.log(dt+', Login Success, '+email+', '+user_data['user_id']);
	return json({
		success:true,
		a: CreateAuthJwt(user_data),
		r: CreateRefreshJwt(user_data)
	});
}
*/