import Database from 'better-sqlite3';

export const feedbackDB = new Database('feedback.db');
export const metricsDB = new Database('metrics.db');

// Prepare db
feedbackDB
	.prepare(
		'CREATE TABLE IF NOT EXISTS feedback (content TEXT, email TEXT, timestamp TEXT DEFAULT CURRENT_TIMESTAMP);'
	)
	.run();
metricsDB
	.prepare(
		'CREATE TABLE IF NOT EXISTS metrics (page TEXT, timestamp TEXT DEFAULT CURRENT_TIMESTAMP);'
	)
	.run();

const addFeedbackStmt = feedbackDB.prepare(
	'INSERT INTO feedback (content, email) VALUES (@content, @email);'
);

const getFeedbackStmt = feedbackDB.prepare('SELECT * FROM feedback;');

const addPageViewStmt = metricsDB.prepare('INSERT INTO metrics (page) VALUES (@page);');
const getDailyPageViewsStmt = metricsDB.prepare(
	"SELECT COUNT(*) AS count, strftime('%Y-%m-%d', timestamp) day FROM metrics GROUP BY day;"
);
const getMostViewedPageStmt = metricsDB.prepare(
	'SELECT page, count(*) AS count FROM metrics GROUP BY page ORDER BY count DESC;'
);

export const addFeedback = (content: string, email?: string) => {
	addFeedbackStmt.run({ content, email });
};

export const addPageView = (page: string) => {
	addPageViewStmt.run({ page });
};

export const getFeedback = async (): Promise<
	Array<{ content: string; email?: string; timestamp: string }>
> => {
	return getFeedbackStmt.all();
};

export const getDailyPageViews = async (): Promise<Array<{ page: string; imestamp: string }>> => {
	return getDailyPageViewsStmt.all();
};

export const getMostViewedPage = async (): Promise<Array<{ page: string; imestamp: string }>> => {
	return getMostViewedPageStmt.all();
};
