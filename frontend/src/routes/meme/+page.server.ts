import type { ServerPageLoad, Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';
import { CreatePostAndUpload } from '$lib/server/create_post.ts';

export const load: ServerPageLoad = async({ locals }) => {
	if(!locals.logged_in){redirect(307, '/login')}
	return locals;
}

export const actions:Actions = {
	upload: async({ locals, request, fetch, clientAddress }) => {
		if(!locals.logged_in){redirect(307, '/login')}

		const form_data = await request.formData();
		const template_id = Number.parseInt(form_data.get('id'));
		const file = form_data.get('file');
		
		const post_link = await CreatePostAndUpload(locals.user_id, file, {mime: 'image/svg+xml'});
		if(post_link){
			redirect(307, '/view/'+post_link);
			//return '/view/'+post_link;
		} else {
			fail(400, {err_msg: 'There was an unknown problem uploading the file'});
		}

		//if(file && file.size > 0) {
		//	//console.log(file);
		//
		//	if(file.size > MAX_FILE_SIZE){
		//		return fail(400, {err_msg:'File is too large, over '+(MAX_FILE_SIZE/1024/1024).toFixed(0)+' MB.'});
		//	}
		//	let verified_type = await fileTypeFromBlob(file);
		//	if(!ALLOWED_MIME_TYPES.includes(verified_type.mime)) {
		//		//console.log('FAILED type');
		//		//console.log(verified_type);
		//		return fail(400, {err_msg:'Unsupported file type'});
		//	}
		//
		//	let id = await CreatePostAndUpload(locals.user_id, file, verified_type);
		//	if(id) {
		//		redirect(307, '/view/' + id);
		//		//return {success:true, post_id:id}
		//	} else {
		//		return fail(400, {err_msg: 'There was an unknown problem uploading the file'})
		//	}
		//}
	}
} satisfies Actions;