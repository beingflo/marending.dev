import Database from 'better-sqlite3';

export const db = new Database('feedback.db');

// Prepare db
const stmt = db.prepare(
	'CREATE TABLE IF NOT EXISTS feedback (content TEXT, email TEXT, timestamp TEXT DEFAULT CURRENT_TIMESTAMP);'
);
stmt.run();

const addFeedbackStmt = db.prepare(
	'INSERT INTO feedback (content, email) VALUES (@content, @email);'
);

const getFeedbackStmt = db.prepare('SELECT * FROM feedback;');

export const addFeedback = (content: string, email?: string) => {
	addFeedbackStmt.run({ content, email });
};

export const getFeedback = async (): Promise<
	Array<{ content: string; email?: string; timestamp: string }>
> => {
	return getFeedbackStmt.all();
};
