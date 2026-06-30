import {
	number as req_number,
	strictTuple as req_tuple,
	array as req_array,
	optional
} from 'valibot';
import { query, getRequestEvent } from '$app/server';
import * as Internal from '$lib/server/posts.ts';

// NOTE: This relies on Svelte Remote Functions, which is EXPERIMENTAL
// But they seem custom-designed to support server-side functions for components
// See more: https://svelte.dev/docs/kit/remote-functions

// NOTE: We're using the live refresh version, which may be flakey
//export const getViews = query(req_number(), async (post_id) => {
//	return (await Internal.GetViewVotes(post_id)).views;
//});

export const getViews = query('unchecked', async ():number => {
	const { params, cookies, fetch } = getRequestEvent();
	const auth_token = cookies.get('ic_auth');
	
	const headers = {'Content-Length':'0'} // Causes problems if not set manually
	if(auth_token){headers['Authorization'] = 'Bearer ' + auth_token}

	return fetch(process.env.IC_LOC_INT+'/posts/p/'+params['id']+'/views',{headers:headers}).then(r=>r.json(),()=>{});
});

export const getVotes = query('unchecked', async ():number => {
	const { params, cookies, fetch } = getRequestEvent();
	const auth_token = cookies.get('ic_auth');
	
	const headers = {'Content-Length':'0'} // Causes problems if not set manually
	if(auth_token){headers['Authorization'] = 'Bearer ' + auth_token}

	return fetch(process.env.IC_LOC_INT+'/posts/p/'+params['id']+'/votes',{headers:headers}).then(r=>r.json(),()=>{});
});


export const liveViews = query.live('unchecked', async function* () {
	console.log("Read live");
	const { params, cookies, fetch } = getRequestEvent();
	const auth_token = cookies.get('ic_auth');
	
	const headers = {'Content-Length':'0'} // Causes problems if not set manually
	if(auth_token){headers['Authorization'] = 'Bearer ' + auth_token}

	while (true) {
		const req = fetch(
			process.env.IC_LOC_INT+'/posts/p/'+params['id']+'/views',
			{headers:headers}
		).then((r)=>{return r.json()},()=>{return null});
		if(req) {yield (req)}
		await new Promise((resolve) => setTimeout(resolve, 15000));
	}
});

// NOTE: We're using the live refresh version, which may be flakey
//export const getVotes = query(req_number(), async (post_id) => {
//	return (await Internal.GetViewVotes(post_id)).votes;
//});
export const liveVotes = query.live(req_number(), async function* (post_id) {
	const { params, cookies, fetch } = getRequestEvent();
	const auth_token = cookies.get('ic_auth');
	
	const headers = {'Content-Length':'0'} // Causes problems if not set manually
	if(auth_token){headers['Authorization'] = 'Bearer ' + auth_token}

	while (true) {
		const req = fetch(
			process.env.IC_LOC_INT+'/posts/p/'+params['id']+'/votes',
			{headers:headers}
		).then((r)=>{return r.json()},()=>{});
		if(req) {yield (req)}
		await new Promise((resolve) => setTimeout(resolve, 15000));
	}
});

export const getMyVote = query(req_tuple([req_number(), optional(req_number())]), async (args):number => {
	const post_id = args[0];
	const user_id = args[1];
	if(user_id){
		return Internal.GetMyVote(post_id, user_id);
	} else {
		return 0
	}
});

export const isFavPost = query(req_tuple([req_number(), optional(req_number())]), async (args):bool => {
	const post_id = args[0];
	const user_id = args[1];
	if(user_id) {
		return Internal.IsFavPost(post_id, user_id);
	} else {
		return false
	}
});

export const get_badges = query(req_array(req_number()), async (ids):any => {
	const { params, cookies, fetch } = getRequestEvent();
	const auth_token = cookies.get('ic_auth');
	
	const body = JSON.stringify(ids);

	const headers = {
		'Content-Length':body.length,
		'Content-Type':'application/json',
	}
	if(auth_token){headers['Authorization'] = 'Bearer ' + auth_token}

	return fetch(process.env.IC_LOC_INT+'/users/ub',{
		method: 'POST',
		headers:headers,
		body: body
	}).then(r=>r.json(),()=>{}).then(r=>{
		const result = new Map();
		for(let i of r) {
			result.set(i.id, i);
		}
		return result;
	},()=>{});
});