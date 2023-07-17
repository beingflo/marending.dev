import data from './matches.json';

const mapped = (data as Array<any>).map((d: any) => ({
	...d,
	startDate: new Date(d.startDate),
	endDate: new Date(d.endDate),
}));

export const getMatches = () => mapped;
