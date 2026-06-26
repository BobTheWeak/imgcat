import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			// What. fucking. garbage. Whatever pulls envvars is opinionated and will not even
			// run if it detects other envvars sharing that prefix. Because it's "dangerous".
			// If you think this is a problem (and it's not) then log a warning. Panicking is
			// utterly inappropreate. Go fuck yourself. Opinionated software is cancer.
			envPrefix: 'IC_FRONTEND_'
		}),
		experimental: {
			remoteFunctions: true
		},
	},
	compilerOptions: {
		experimental: {
			async: true
		}
	}
};

export default config;
