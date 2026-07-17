// The cookie library decodes a cookie into different field names than
// Svelte's cookie.set(name, val, opts) accepts, so we have to translate
// NOTE: Used in /auth/cb, /signup, and hooks.server.ts
export function cookie_to_svelte_opts(cookie_obj) {
	const result = {};
	if(cookie_obj['Path']) { result['path'] = cookie_obj['Path'] }
	if(cookie_obj['Max-Age']) { result['maxAge'] = Number.parseInt(cookie_obj['Max-Age']) }
	if(cookie_obj['SameSite']) { result['sameSite'] = cookie_obj['SameSite'] }
	return result;
}