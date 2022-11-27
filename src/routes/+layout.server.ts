import { addPageView } from '$lib/db';

export const load = ({ url }: { url: { pathname: string } }) => {
	if (
		url.pathname.startsWith('/notes/') ||
		['/', '/about', '/metrics'].indexOf(url.pathname) > -1
	) {
		addPageView(url.pathname);
	}
};
