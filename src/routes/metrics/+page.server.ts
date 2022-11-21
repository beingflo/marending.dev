import { getPageViews } from '$lib/db';

export const load = () => {
	return { pageViews: getPageViews() };
};
