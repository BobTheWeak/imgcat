import type { LayoutServerLoad } from './$types';
import 'dotenv/config';

export const load: LayoutServerLoad = async({ locals }) => {
	return {
		...locals,
		imageBaseUrl: process.env.IMAGE_BASE_URL || 'http://localhost:9000/imgcat-dev'
	};
}