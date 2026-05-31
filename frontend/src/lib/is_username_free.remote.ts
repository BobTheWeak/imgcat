import * as v from 'valibot';
import { query, getRequestEvent } from '$app/server';

// TODO: Allow accented characters too (that'll be a pain)
// NOTE: ":" is reserved as a special character. ex: Mod:MyUserName
const USERNAME_REGEX = new RegExp("^[A-Za-z0-9"+RegExp.escape(" +-*.,!?_=~")+"]+$");
// Exactly 2 uppercase characters
const COUNTRY_REGEX = new RegExp("^[A-Z][A-Z]$");

export const IsUsernameFree = query(v.string(), async(username) => {

	// The DB allows 40 characters, but this can be expanded if needed
	if(username.length < 4) {return "Username is too short (at least 4)"}
	if(username.length > 40) {return "Username is too long (40 or less)"}

	const lc = username.toLowerCase();

	// Check symbols
	if(!USERNAME_REGEX.test(lc)) {return "Improper symbols"}

	// Banned words
	if(lc.indexOf('imgcat') !== -1) {return "Improper username"}
	if(lc.indexOf('admin') !== -1) {return "Improper username"}
	if(lc.indexOf('moderator') !== -1) {return "Improper username"}
	
	const { fetch, cookies } = getRequestEvent();
	const sjwt = cookies.get('ic_signup');
	if(sjwt){
		const res = await fetch('/api/auth/namefree?u='+encodeURI(username), {
			headers: {
				'Authorization': "Bearer " + cookies.get('ic_signup')
			}
		});
		if(res.status === 200) {
			// Body is either "1" or "0"
			return await res.text() === "1";
		}
	}
	// TODO: Error handling here
	// To the user, I think ignoring this is fine. "Sorry, we can't check if
	// your username is available", is not helpful, fixable, or reassuring.
	// But this is something backend folks should know about b/c it's probably
	// a bug or a cert problem
});