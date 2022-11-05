import { db } from '$lib/db';

export const load = async () => {
	const stmt = db.prepare('SELECT number, text from views;');
	const rows = stmt.all();

	return {
		rows,
	};
};
