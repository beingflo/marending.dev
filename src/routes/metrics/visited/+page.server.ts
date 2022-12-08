import { getMostVisited } from '$lib/db';

export const load = () => {
	return { mostVisited: getMostVisited() };
};
