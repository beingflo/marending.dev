import { getDailyPageViews, getMostViewedPage } from '$lib/db';

export const load = () => {
	return { pageViews: getDailyPageViews(), mostViewed: getMostViewedPage() };
};
