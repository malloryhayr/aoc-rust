import type { PageLoad } from './$types';
import init from 'rust';

export const load: PageLoad = async ({ params }) => {
	await init();

	return {};
};

export const ssr = false;
