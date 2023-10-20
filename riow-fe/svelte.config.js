import adapter from '@sveltejs/adapter-auto';
import adapterStatic from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [vitePreprocess({})],

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter:
			process.env.ADAPTER === 'static'
				? adapterStatic({
						// default options are shown. On some platforms
						// these options are set automatically — see below
						pages: '../shuttle/assets',
						assets: '../shuttle/assets',
						fallback: undefined,
						precompress: false,
						strict: true
				  })
				: adapter(),
		paths:
			process.env.ADAPTER === 'static'
				? {
						base: '/ui',
						relative: false
				  }
				: undefined
	}
};

export default config;
