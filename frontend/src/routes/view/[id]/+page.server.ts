import type { PageServerLoad, Actions } from './$types';
import { fail, error } from '@sveltejs/kit';


////////////////////////////////////////////////////////
//   THIS SHOULD BE DEPRECATED - USE A MICROSERVICE   //
import { GetPost, GetMyVote } from '$lib/server/posts.ts';
import { GetPostIdByLink, SetVote, ToggleFavPost } from '$lib/server/posts.ts';
import { CreateComment } from '$lib/server/create_comment.ts';
////////////////////////////////////////////////////////


const TAG_REGEX = /^[\w\s\-,]+$/
const COMMENT_REGEX = /^[\w\s\-,./?;:'"!@#$%^&*\(\)_=+\\\|]*$/


export const load:PageServerLoad = async({ params, locals }) => {
	const post = await GetPost(params['id'], locals.content_level, locals.user_id);

	// A bad link? Trying to scrape the site? Or hidden b/c of a content violation?
	// Whatever the reason, this user cannot access this content
	if(!post){error(404, "Post not found")}

	// Internally, we're pulling from the 'link_v1' column
	for(let i in post.img) {
		post.img[i].link = '/api/img/' + post.img[i].link
		//HARDCODED: 0:Unknown, 1:raster image, 2:vector image, 3:animation, 4:video
		post.img[i].type = ['unknown', 'image', 'svg', 'image', 'video'][post.img[i].type]
	}

	return {
		post: post,
	};
}

// Most of the actions on this page are form submits (below), because
// they can be triggered without enabling javascript. Users should be
// able to look at images w/o 
export const actions:Actions = {
	// DEPRECATED: See CommentBox. We're moving comments into a common control
	// with remote functions, not this form. top-level comments are doing that now,
	// But nested replies are still using this.
	// NOTE: The benefit of forms is they work w/o JS, which is nice, but they're
	// not really compatible with Controls that need independent ajax stuff.
	comment: async({params, locals, request}) => {
		if(!locals.logged_in) {return error(403)}

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
	},

	upvote: async({params, locals, cookies, request, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);

		// TODO: Use a microservice & send the auth_token
		let new_votes = await SetVote(post_id, locals.user_id, 1);
		return {success:true};
	},
	dnvote: async({params, locals, cookies, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);

		// TODO: Use a microservice & send the auth_token
		let new_votes = await SetVote(post_id, locals.user_id, 5);
		return {success:true};
	},
	fav: async({params, locals, cookies, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);

		// TODO: Use a microservice & send the auth_token
		await ToggleFavPost(post_id, locals.user_id);
		return {success:true};
	},

	tag: async({params, locals, cookies, request, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);
		const data = await request.formData();


		// Check Maturity
		let fetch_params = new URLSearchParams();
		const mature_num = {'kid':0,'std':1,'sfw':2,'nsfw':3,'ill':4}[data.get('maturity')];
		if(mature_num) {
			fetch_params.append('m', mature_num);
			if(data.get('is_sexual')){fetch_params.append('s',1)}
			if(data.get('is_gore')){fetch_params.append('g',1)}
			if(data.get('is_trauma')){fetch_params.append('t',1)}
		}

		// Make API call
		let maturity_fetch = null;
		if(fetch_params.size>0){
			let url = `/api/smact/vote_mature/${post_id}?${fetch_params.toString()}`;
			maturity_fetch = fetch(url, {
				method: 'POST',
				headers: {
					'Content-Length': '0', // Causes problems if not set manually
					'Authorization': 'Bearer ' + auth_token
				}
			});
		}


		// Check Categories
		fetch_params = new URLSearchParams();
		if(data.get('cat_animals')){fetch_params.append('a',1)}
		if(data.get('cat_artists')){fetch_params.append('c',1)}
		if(data.get('cat_selfies')){fetch_params.append('s',1)}
		if(data.get('cat_news')){fetch_params.append('n',1)}
		if(data.get('cat_politics')){fetch_params.append('p',1)}
		if(data.get('cat_ai')){fetch_params.append('r',1)}

		// Make API call
		let cat_fetch = null;
		if(fetch_params.size>0){
			let url = `/api/smact/vote_category/${post_id}?${fetch_params.toString()}`;
			cat_fetch = fetch(url, {
				method: 'POST',
				headers: {
					'Content-Length': '0', // Causes problems if not set manually
					'Authorization': 'Bearer ' + auth_token
				}
			});
		}


		// Check tags
		fetch_params = new URLSearchParams();
		data.get('tags')?.replaceAll(/\s+/g, ' ')
			.split(',')
			.map(t=>t.trim())
			.filter(t=>TAG_REGEX.test(t))
			.forEach(t=>fetch_params.append('t',t));

		// Make API call
		let tag_fetch = null;
		if(fetch_params.size>0){
			let url = `/api/smact/vote_tag/${post_id}?${fetch_params.toString()}`;
			tag_fetch = fetch(url, {
				method: 'POST',
				headers: {
					'Content-Length': '0', // Causes problems if not set manually
					'Authorization': 'Bearer ' + auth_token
				}
			});
		}


		// Resolve the multiple-independent calls we need to make
		[maturity_fetch, cat_fetch, tag_fetch] = await Promise.all([
			maturity_fetch, cat_fetch, tag_fetch
		]);


		// Check the results
		let result=true;

		if(maturity_fetch && maturity_fetch.status!==200) {
			result=false;
		}
		if(cat_fetch && cat_fetch.status!==200) {
			result=false;
		}
		if(tag_fetch && tag_fetch.status!==200) {
			result=false;
		}
	
		return {form:'tag', success:result};
	},

	mute: async({params, locals, cookies, request, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);
		const data = await request.formData();
		

		
		let result = true;
		return {'form':'report', 'success':result};
	},
	snooze: async({params, locals, cookies, request, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);
		const data = await request.formData();
		
		
		
		let result = true;
		return {'form':'report', 'success':result};
	},
	block: async({params, locals, cookies, request, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);
		const data = await request.formData();
		
		
		
		let result = true;
		return {'form':'report', 'success':result};
	},

	report: async({params, locals, cookies, request, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);
		const data = await request.formData();
		
		let comment = data.get('comment')?.trim();
		if(!COMMENT_REGEX.test(comment)) {
			// TODO: It might be a more streamlined experience to just strip anything invalid
			return {'form':'report', 'success':false, message:'Invalid symbols'};
		}
		if(comment.length===0) {
			return {'form':'report', 'success':false, message:'Comments are required, and help us respond faster', comment:comment};
		}
		comment = encodeURI(comment);
		
		// Make API call
		let url = `/api/smact/vote_review/${post_id}?c=${comment}`;
		let report_fetch = await fetch(url, {
			method: 'POST',
			headers: {
				'Content-Length': '0', // Causes problems if not set manually
				'Authorization': 'Bearer ' + auth_token
			}
		});

		let result = true;
		if(report_fetch && report_fetch?.status!==200) {
			result=false;
		}
		
		return {'form':'report', 'success':result};
	},

	report_anon: async({params, locals, request, fetch}) => {
		// NOTE: If they ARE logged in, they shouldn't be using this one
		// TODO: Maybe just forward them? But they shouldn't be here.
		if(locals.logged_in) {return error(403)}
		
		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);
		const data = await request.formData();
		
		let comment = data.get('comment')?.trim();
		if(!COMMENT_REGEX.test(comment)) {
			// TODO: It might be a more streamlined experience to just strip anything invalid
			return {'form':'report_anon', 'success':false, message:'Invalid symbols'};
		}
		if(comment.length===0) {
			return {'form':'report_anon', 'success':false, message:'Comments are required, and help us respond faster', comment:comment};
		}
		comment = encodeURI(comment);

		// Make API call
		let url = `/api/smact/anon_review/${post_id}?c=${comment}`;
		let report_fetch = await fetch(url, {
			method: 'POST',
			headers: {
				'Content-Length': '0', // Causes problems if not set manually
			}
		});

		let result = true;
		if(report_fetch && report_fetch?.status!==200) {
			result=false;
		}
		
		return {'form':'report_anon', 'success':result}
	},
}