import { getDailyPageViews } from '$lib/db';

export const load = () => {
	return { pageViews: getDailyPageViews() };
};
