import { addPageView } from '$lib/db';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const load = (params: any) => {
	// For load testing without distorting the metrics
	if (params.request.headers.get('no-track')) {
		return;
	}

	if (
		params.url.pathname.startsWith('/notes/') ||
		params.url.pathname.startsWith('/metrics/') ||
		['/', '/about', '/projects'].indexOf(params.url.pathname) > -1
	) {
		addPageView(params.url.pathname, params.request.headers.get('referer'));
	}
};

export const prerender = true;
