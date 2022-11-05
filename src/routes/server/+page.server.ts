import { db } from '$lib/db';

export const load = () => {
	const stmt = db.prepare('INSERT INTO views (number, text) VALUES (?, ?);');
	console.log(stmt.run(123, 'test'));

	return {
		test: 'world',
	};
};
