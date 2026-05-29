
import type { PageLoad } from './$types';

export const load: PageLoad = ({ url }) => {
	const result = {};

	// If there's an error message in the URL (returned by auth), show that
	if((new URL(url)).searchParams.has('error_msg')) {
		result.error_msg = (new URL(url)).searchParams.get('error_msg');
	}

	return result;
};
