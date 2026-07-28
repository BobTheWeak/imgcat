
import type { PageLoad } from './$types';

export const load: PageLoad = ({ url, cookies }) => {
	const result = {};

	// If there's an error message in the URL (returned by auth), show that
	if((new URL(url)).searchParams.has('error_msg')) {
		result.error_msg = (new URL(url)).searchParams.get('error_msg');
		// TODO: Can we strip this from the URL, so it won't show on reload?
	}

	// If this is a private/test server, check for the "ic_private" cookie
	if(process.env.IC_PVT_SVR) {
		result.login_allowed = false;

		// Check the cookie, and see if it contains this private server's code
		const pvt_auth = cookies.get('ic_private');
		if(pvt_auth) {
			// Support a semicolon-separated list of values, to support multiple servers
			if(pvt_auth.split(";").includes(process.env.IC_PVT_SVR)) {
				result.login_allowed = true;
			}
		}
	} else {
		result.login_allowed = true;
	}

	return result;
};
