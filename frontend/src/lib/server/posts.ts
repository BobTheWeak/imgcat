
//import mariadb from 'mariadb';
//import 'dotenv/config';
import { error } from '@sveltejs/kit';

import { query, row } from '$lib/server/dbpool.ts';



export async function GetMyPosts(user_id:number, pg_num:number, pg_size:number):any {
	if(!user_id){return null}

	return query(
		"CALL Posts.GetMyPosts(?,?,?);",
		[user_id, (pg_num*pg_size) || 0, pg_size || 50],
		(r)=>r[0]
	);
}

export async function GetMyFavs(user_id:number, pg_num:number, pg_size:number):any {
	if(!user_id){return null}

	return query(
		"CALL Posts.GetMyFavs(?,?,?);",
		[user_id, (pg_num*pg_size) || 0, pg_size || 50],
		(r)=>r[0]
	);
}

export async function GetRecentPage(content_level:number, pg_num:number, pg_size:number):any {
	if(!content_level){return null}

	return query(
		"CALL Posts.GetRecentPage(?,?,?);",
		[content_level, (pg_num*pg_size) || 0, pg_size || 50],
		(r)=>r[0]
	);
}

export async function GetViralPage(content_level:number, pg_num:number, pg_size:number):any {
	if(!content_level){return null}

	return query(
		"CALL Posts.GetViralPage(?,?,?);",
		[content_level, (pg_num*pg_size) || 0, pg_size || 50],
		(r)=>r[0]
	);
}

export async function GetHomePage(content_level:number, pg_num:number, pg_size:number):any {
	if(!content_level){return null}

	return query(
		"CALL Posts.GetHomePage(?,?,?);",
		[content_level, (pg_num*pg_size) || 0, pg_size || 50],
		(r)=>r[0]
	);
}

export async function GetPost(post_link:string, content_level:number, user_id:number|null):any {
	if(!post_link || !content_level){return null}

	return query(
		"CALL Posts.GetPost(?,?,?,true);",
		[post_link, content_level, user_id],
		(r)=>{
			let result = r[0][0];
			// It comes back as BigInt
			result['user_id'] = Number(result['user_id']);
			// Fold the second resultset into img=[]
			result['img'] = r[1];
			// Goddamned timezones + Javascript + MySQL + a shit-ass library doing silent
			// conversions from UTC into local fuckery. ALWAYS USE UTC. LOCAL IS WRONG.
			result['time'] = new Date(Number(result.time)*1000)

			return result;
		}
	);
}

export async function GetPostIdByLink(post_link:string):number {
	if(!post_link){return null}

	return row(
		"SELECT Posts.GetPostIdByLink(?);",
		[post_link]
	);
}



export async function GetViewVotes(post_id:number):number {
	return row(
		"CALL Posts.GetViewVotes(?);",
		[post_id],
		(r)=>{
			return {
				views: r[0],
				votes: r[1]
			}
		}
	);
}

export async function GetMyVote(post_id:number, user_id:number):number {
	return row(
		"SELECT Posts.GetMyVote(?,?);",
		[post_id, user_id]
	);
}

export async function IsFavPost(post_id:number, user_id:number):bool {
	return row(
		"SELECT Actions.IsFavPost(?,?);",
		[user_id, post_id], // WARNING: User_id first
		(r)=>r===1
	);
}

export async function SetView(post_id:number, user_id:number):number {
	return row(
		"SELECT Posts.SetView(?,?);",
		[post_id, user_id],
		(r)=>r[0]
	);
}

// SetVote will set the vote to whatever specified, or toggle it if its already set
export async function SetVote(post_id:number, user_id:number, value:number):number {
	// 0: unset, 1:Upvote, 5:Downvote
	if(value===0 || value===1 || value===5){
		return row(
			"SELECT Posts.SetVote(?,?,?);",
			[post_id, user_id, value]
		);
	}
}

export async function ToggleFavPost(post_id:number, user_id:number, folder_name:string|undefined): bool {
	// NOTE: The UI doesn't support folder names yet
	if(!folder_name) {
		return row(
			"SELECT Actions.ToggleFavPost(?,?);",
			[user_id, post_id] // WARNING: User_id first
		);
	} else {
		return row(
			"SELECT Actions.ToggleFavPost(?,?,?);",
			[user_id, post_id, folder_name] // WARNING: User_id first
		);
	}
}

export async function SetPostPublic(post_id:number, user_id:number, is_pub:bool|undefined) {
	// NOTE: If undefined, then it'll toggle states instead of setting
	if(is_pub!==undefined) {
		row(
			"CALL Posts.SetPostPublic(?,?,?);",
			[user_id, post_id, is_pub] // WARNING: User_id first
		);
	} else {
		row(
			"CALL Posts.SetPostPublic(?,?);",
			[user_id, post_id] // WARNING: User_id first
		);
	}
}