import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { form, getRequestEvent } from '$app/server';
import { getDbConn, array as sql_array } from '$lib/server/dbpool.ts';

// NOTE: This relies on Svelte Remote Functions, which is EXPERIMENTAL
// But they seem custom-designed to support server-side functions for components
// See more: https://svelte.dev/docs/kit/remote-functions
// 


export const voteMature = form(v.object({
		post_id: v.integer(),
		mature: v.picklist(['kid','std','18+','xxx','ill']),
		is_sexual: v.optional(v.boolean(), false),
		is_gore: v.optional(v.boolean(), false),
		is_trauma: v.optional(v.boolean(), false),
	}), async({post_id, mature, is_sexual, is_gore, is_trauma}) => {

	const { locals, fetch } = getRequestEvent();

	if(locals?.user_id) {
		const mature_num = {'kid':0,'std':1,'18+':2,'xxx':3,'ill':4}[mature];
		let url = `/api/smact/vote_mature/${post_id}?m=${mature_num}`;
		if(is_sexual){url+='&s=1'}
		if(is_gore){url+='&g=1'}
		if(is_trauma){url+='&t=1'}

		const res = await fetch(url, {
			method: 'POST',
			headers: {'Content-Length': '0'} // Causes problems if not set manually
		});
		return { success:true }
	} else {
		// Anon user. They should not have gotten here
		return error(403);
	}
	
});


export const voteCategory = form(v.object({
		post_id: v.integer(),
		is_politics: v.optional(v.boolean(), false),
		is_thirst_trap: v.optional(v.boolean(), false),
		is_creator: v.optional(v.boolean(), false),
	}), async({post_id, is_politics, is_thirst_trap, is_creator}) => {

	const { locals, fetch } = getRequestEvent();

	if(locals?.user_id) {
		let url = `/api/smact/vote_category/${post_id}`;
		let sep = '?';
		if(is_politics){url+=sep+'s=1';sep='&';}
		if(is_thirst_trap){url+=sep+'g=1';sep='&';}
		if(is_creator){url+=sep+'t=1';sep='&';}

		// Only submit if they're submitting at least 1 tag
		if(sep=='&') {
			const res = await fetch(url, {
				method: 'POST',
				headers: {'Content-Length': '0'} // Causes problems if not set manually
			});
		}

		return { success:true }
	} else {
		// Anon user. They should not have gotten here
		return error(403);
	}
	
});


export const voteTag = form(v.objectWithRest({
		post_id: v.integer(),
		is_politics: v.optional(v.boolean(), false),
		is_ttrap: v.optional(v.boolean(), false),
		is_creator: v.optional(v.boolean(), false),
	}, v.pipe(v.string(), v.maxLength(40))), async({post_id, is_politics, is_ttrap, is_creator, ...tags}) => {

	const { locals, fetch } = getRequestEvent();

	if(locals?.user_id) {
		// This requires 2 API calls handling: fixed categories & free-form tags
		let url = `/api/smact/vote_category/${post_id}`;
		let sep = '?';
		if(is_politics){url+=sep+'p=1';sep='&';}
		if(is_ttrap){url+=sep+'t=1';sep='&';}
		if(is_creator){url+=sep+'c=1';sep='&';}

		// If there's at least one category
		if(sep=='&') {
			const res = await fetch(url, {
				method: 'POST',
				headers: {'Content-Length': '0'} // Causes problems if not set manually
			});
		}

		// Start the second REST call
		// TODO: Do both of these as parallel promises, not this serial await stuff...
		url = `/api/smact/vote_tag/${post_id}`;
		sep = '?';
		for(let key in tags) {
			// WARNING: We're accepting user input here.
			// TODO: Better validation, XML checking, non-printing chars, etc.
			url+=sep+'t='+encodeURIComponent(tags[key]);
			sep = '&';
		}

		// If there's at least one suggested tag
		if(sep=='&') {
			const res = await fetch(url, {
				method: 'POST',
				headers: {'Content-Length': '0'} // Causes problems if not set manually
			});
		}

		return { success:true }
	} else {
		// Anon user. They should not have gotten here
		return error(403);
	}
	
});


export const pingMod = form(v.object({
		post_id: v.integer(),
		comment: v.pipe(v.string(), v.minLength(10), v.maxLength(200)),
	}), async({post_id, comment}) => {

	const { locals, fetch } = getRequestEvent();
	
	// WARNING: We're accepting user input here.
	// TODO: Better validation, XML checking, non-printing chars, etc.
	const comment_encoded = encodeURIComponent(comment);

	if(locals?.user_id) {
		const url = `/api/smact/vote_review/${post_id}?c=${comment_encoded}`;
		const res = await fetch(url, {
			method: 'POST',
			headers: {'Content-Length': '0'} // Causes problems if not set manually
		});

		return { success:true }
	} else {
		const url = `/api/smact/anon_review/${post_id}?c=${comment_encoded}`;
		const res = await fetch(url, {
			method: 'POST',
			headers: {'Content-Length': '0'} // Causes problems if not set manually
		});

		return { success:true }
	}
	
});

