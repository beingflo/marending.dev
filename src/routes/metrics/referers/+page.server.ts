import { getReferers } from '$lib/db';

type Row = { referrer: string; count: number };

export const load = async () => {
	const referers = await getReferers();

	const getHost = (referrer: string) => {
		const url = new URL(referrer);
		return url.host;
	};

	const mappedReferers = referers
		.filter((row: Row) => row.referrer !== null)
		.map((row) => ({ referrer: getHost(row.referrer), count: row.count }));

	const aggregatedReferers: Row[] = [];

	mappedReferers.reduce((res: Record<string, Row>, value: Row) => {
		if (!res[value.referrer]) {
			res[value.referrer] = { referrer: value.referrer, count: value.count };
			aggregatedReferers.push(res[value.referrer]);
		} else {
			res[value.referrer].count += value.count;
		}
		return res;
	}, {});

	return {
		referers: aggregatedReferers,
	};
};
