import { addPageView } from '$lib/db';

export const load = ({ url }: { url: { pathname: string } }) => {
	addPageView(url.pathname);
};
