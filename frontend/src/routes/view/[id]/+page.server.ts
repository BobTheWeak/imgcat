import type { PageServerLoad, Actions } from './$types';
import { fail } from '@sveltejs/kit';

import { GetPost } from '$lib/server/posts.ts';
import { CreateComment } from '$lib/server/create_comment.ts';

export const load:PageServerLoad = async({ params, locals }) => {
	let post = await GetPost(params['id'], locals.content_level, locals.user_id);
	
	// A bad link? Trying to scrape the site? Or hidden b/c of a content violation?
	// Whatever the reason, this user cannot access this content
	if(!post){return null}
	
	// Internally, we're pulling from the 'link_v1' column
	// TODO: Make this configurable (use a microservice/proxy)
	//let BASE_URL = process.env.THIS_NEEDS_TO_BE_CONFIGURABLE
	for(let i in post.img) {
		post.img[i].link = '/api/img/' + post.img[i].link
	}

	if(post) {
		return {
			'post': post
		};
	} else {
		return null;
	}
}

// DEPRECATED: See CommentBox. We're moving comments into a common control
// with remote functions, not this form. top-level comments are doing that now,
// But nested replies are still using this.
// NOTE: The benefit of forms is they work w/o JS, which is nice, but they're
// not really compatible with Controls that need independent ajax stuff.
export const actions:Actions = {
	comment: async({params, locals, request}) => {
		let data = await request.formData();
		let comment_id = await CreateComment({
			user_id: locals.user_id,
			post_id: params['id'],
			reply_to: data.get('reply_to'), //undefined is OK
			link: undefined, // TODO: Add linking
			comment: data.get('comment')
		});
		if(comment_id) {
			return {success:true}
		} else {
			return fail(400);
		}
	}
}