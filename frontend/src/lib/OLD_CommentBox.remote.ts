import {
	strictTuple as req_tuple,
	nullable as req_nullable,
	number as req_number,
	string as req_string
} from 'valibot';
import { error } from '@sveltejs/kit';
import { query } from '$app/server';
import { getDbConn, array as sql_array } from '$lib/server/dbpool.ts';

// NOTE: This relies on Svelte Remote Functions, which is EXPERIMENTAL
// But they seem custom-designed to support server-side functions for components
// See more: https://svelte.dev/docs/kit/remote-functions
// 

export const getPostComments = query(req_number(), async (post_id) => {
	let conn;
	try {
		conn = await getDbConn();
		return conn.query({
				sql:"CALL Comments.GetPostComments(?);"
			}, [post_id])
			.then((r)=>{
				if(r[0]?.length > 0) {
					// NOTE: The returned data is unordered & unnested
					// Currently: Sort by time (earliest first)
					// TODO: Incorporate upvotes/rating
					// TODO: Verify nesting code
					// NOTE: We're sorting once, up top, and maintaining a consistent
					// sort order throughout the process. Even multi-nested 
					r[0].sort((a,b)=>{return a.time - b.time});

					const root = [];
					const nested = {};

					for(const c of r[0]) {
						if(c['reply_to']) {
							if(!nested[c['reply_to']]) {
								nested[c['reply_to']]=[];
							}
							nested[c['reply_to']].push(c)
						} else {
							root.push(c);
						}
					}

					return {root, nested};
				} else {
					return {root:null, nested:null};
				}
			})
			.catch((e)=>{
				return null;
			});
	} catch(e) {
		console.log(e); // NOTE: Logs to server, not client
		error(500, 'Data error');
	} finally {
		if(conn){await conn.release()}
	}
});


export const createComment = query(req_tuple([
		req_number(),
		req_number(),
		req_nullable(req_number()),
		req_nullable(req_string()),
		req_string()
	]), async (args) => {
	const user_id:number = parseInt(args[0]);
	const post_id:number = parseInt(args[1]);
	const reply_to:number|null = parseInt(args[2])||null;
	// TODO: This is the CHAR(12) ref to add a picture along with your message
	const img_link:string|null = null; // args[3] reserved
	const message:string = args[4];

	return sql_array(
		"SELECT Posts.CreateComment(?,?,?,?,?);",
		[user_id, post_id, reply_to, img_link, message],
		(r)=>r[0][0]
	);
});