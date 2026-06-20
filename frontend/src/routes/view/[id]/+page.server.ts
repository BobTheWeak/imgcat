import type { PageServerLoad, Actions } from './$types';
import { fail, error } from '@sveltejs/kit';
import { get_badges } from './actions.remote.ts';

////////////////////////////////////////////////////////
//   THIS SHOULD BE DEPRECATED - USE A MICROSERVICE   //
import { GetPost, GetMyVote, IsFavPost } from '$lib/server/posts.ts';
import { GetPostIdByLink, SetVote, ToggleFavPost, SetPostPublic } from '$lib/server/posts.ts';
import { CreateComment } from '$lib/server/create_comment.ts';
////////////////////////////////////////////////////////


const TAG_REGEX = /^[\w\s\-,]+$/
const COMMENT_REGEX = /^[\w\s\-,./?;:'"!@#$%^&*\(\)_=+\\\|]*$/


export const load:PageServerLoad = async({ params, locals, cookies, fetch }) => {
	const post = await GetPost(params['id'], locals.content_level, locals.user_id);
	const auth_token = cookies.get('ic_auth')

	// A bad link? Trying to scrape the site? Or hidden b/c of a content violation?
	// Whatever the reason, this user cannot access this content
	if(!post){error(404, "Post not found")}

	// Internally, we're pulling from the 'link_v1' column
	for(let i in post.img) {
		post.img[i].link = '/api/img/' + post.img[i].link
		//HARDCODED: 0:Unknown, 1:raster image, 2:vector image, 3:animation, 4:video
		post.img[i].type = ['unknown', 'image', 'svg', 'image', 'video'][post.img[i].type]
	}

	// Users Service (grabbing usernames)
	// NOTE: It gets slightly weird if we don't await here. It's worth it.
	const badges = await get_badges([post.user_id]);
	post.user = badges?.get(post.user_id);

	// Posts Service
	let url = '/api/posts/p/' + params['id'];
	let h = {'Content-Length':'0'} // Causes problems if not set manually
	if(auth_token){h['Authorization']='Bearer '+auth_token}

	// A bunch of things to lazy-load
	const views = fetch(url + '/views', {headers:h}).then(r=>r.json(),()=>{});
	const votes = fetch(url + '/votes', {headers:h}).then(r=>r.json(),()=>{});
	const comment_replies = fetch(url + '/comments', {headers:h}).then(r=>r.json(),()=>{})
		.then(r=>{
			// Two fixes:
			// 1) JSON keys are strings, but we sent them as Numbers (i64)
			// 2) JS dates use ms, but we send UNIX time in secs
			const result = {comments:new Map(), replies:new Map()}

			for(let item of Object.entries(r.comments)) {
				item[1].ts = new Date(Number.parseInt(item[1].ts * 1000));
				result.comments.set(Number.parseInt(item[0]), item[1])
			}
			for(let item of Object.entries(r.replies)) {
				result.replies.set(Number.parseInt(item[0]), item[1])
			}
			return result;
		},()=>{});

	const result = {
		post: post,
		// Lazy-loaded promises
		views: views,
		votes: votes,
		comment_replies: comment_replies,
	}

	//
	if(locals.logged_in) {
		// TODO: Replace with promise APIs
		result['my_vote'] = GetMyVote(post.id, locals.user_id);
		result['is_fav'] = IsFavPost(post.id, locals.user_id);
	}

	return result;
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
	comment: async({params, locals, cookies, fetch, request}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);

		// Get the form data
		const data = await request.formData();
		let reply_to = Number.parseInt(data.get('reply_to'));
		if(Number.isNaN(reply_to)){reply_to=null}
		const comment = data.get('comment');

		// TODO: If comment looks like an ImgCat URL, then do a pic

		// TODO: Use a microservice, not SQL
		let comment_id = await CreateComment({
			user_id: locals.user_id,
			post_id: post_id,
			reply_to: reply_to, //null is OK
			link: null, // TODO: Add linking
			comment: comment,
		});

		if(comment_id) {
			// TODO: At the moment, new comments will magically appear, but collapsed.
			// We need to open the direct (reply_to) parent. Thankfully, we know
			// that all others above it have already been opened.
			return { success:true, comment_id:comment_id, reply_to:data.get('reply_to') }
		} else {
			return fail(400);
		}
	},

	public: async({params, locals, cookies, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);

		// Just toggle whatever it is. But it does support setter values.
		await SetPostPublic(post_id, locals.user_id);
		return {success:true}
	},

	upvote: async({params, locals, cookies, fetch}) => {
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
		
		// TODO:
		
		let result = false;
		return {'form':'report', 'success':result, message:'Not implemented yet'};
	},
	snooze: async({params, locals, cookies, request, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);
		const data = await request.formData();
		
		// TODO:
		
		let result = false;
		return {'form':'report', 'success':result, message:'Not implemented yet'};
	},
	block: async({params, locals, cookies, request, fetch}) => {
		// Check permissions
		if(!locals.logged_in) {return error(403)}
		const auth_token = cookies.get('ic_auth');
		if(!auth_token) {return error(403)}

		// Fetch the post_id
		const post_id = await GetPostIdByLink(params['id']);
		const data = await request.formData();
		
		// TODO:
		
		let result = false;
		return {'form':'report', 'success':result, message:'Not implemented yet'};
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