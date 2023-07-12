import data from './matches.json';

const mapped = data.map((d) => ({
	...d,
	startDate: new Date(d.startDate),
	endDate: new Date(d.endDate),
}));

export const getMatches = () => mapped;
