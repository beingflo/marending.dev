import { addPageView } from '$lib/db';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const load = (params: any) => {
	if (
		params.url.pathname.startsWith('/notes/') ||
		['/', '/about', '/metrics'].indexOf(params.url.pathname) > -1
	) {
		addPageView(params.url.pathname, params.request.headers.get('referer'));
	}
};
