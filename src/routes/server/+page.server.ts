import { db } from '$lib/db';

export const load = () => {
	const stmt = db.prepare('INSERT INTO views (number, text) VALUES (?, ?);');
	stmt.run(123, new Date().toISOString());

	return {
		test: 'world',
	};
};
