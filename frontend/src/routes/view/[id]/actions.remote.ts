import { number as req_number, strictTuple as req_tuple } from 'valibot';
import { error } from '@sveltejs/kit';
import { query, command } from '$app/server';
import { getDbConn } from '$lib/server/dbpool.ts';
import * as Internal from '$lib/server/posts.ts';

// NOTE: This relies on Svelte Remote Functions, which is EXPERIMENTAL
// But they seem custom-designed to support server-side functions for components
// See more: https://svelte.dev/docs/kit/remote-functions
// 


// NOTE: We're using the live refresh version, which may be flakey
//export const getViews = query(req_number(), async (post_id) => {
//	return (await Internal.GetViewVotes(post_id)).views;
//});
export const liveViews = query.live(req_number(), async function* (post_id) {
	while (true) {
		yield (await Internal.GetViewVotes(post_id)).views;
		await new Promise((resolve) => setTimeout(resolve, 15000));
	}
});

// NOTE: We're using the live refresh version, which may be flakey
//export const getVotes = query(req_number(), async (post_id) => {
//	return (await Internal.GetViewVotes(post_id)).votes;
//});
export const liveVotes = query.live(req_number(), async function* (post_id) {
	while (true) {
		yield (await Internal.GetViewVotes(post_id)).votes;
		await new Promise((resolve) => setTimeout(resolve, 15000));
	}
});



export const getMyVote = query(req_tuple([req_number(), req_number()]), async (args):number => {
	const post_id = args[0];
	const user_id = args[1];
	return Internal.GetMyVote(post_id, user_id);
});

export const isFavPost = query(req_tuple([req_number(), req_number()]), async (args):bool => {
	const post_id = args[0];
	const user_id = args[1];
	return Internal.IsFavPost(post_id, user_id);
});
