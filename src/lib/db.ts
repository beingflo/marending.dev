import Database from 'better-sqlite3';

export const feedbackDB = new Database('feedback.db');
export const metricsDB = new Database('metrics.db');

if (feedbackDB.pragma('user_version', { simple: true }) === 0) {
	// Prepare db
	feedbackDB
		.prepare(
			'CREATE TABLE IF NOT EXISTS feedback (content TEXT, email TEXT, timestamp TEXT DEFAULT CURRENT_TIMESTAMP);'
		)
		.run();
	feedbackDB.pragma('user_version = 1');
}

if (metricsDB.pragma('user_version', { simple: true }) === 0) {
	metricsDB
		.prepare(
			'CREATE TABLE IF NOT EXISTS metrics (page TEXT, timestamp TEXT DEFAULT CURRENT_TIMESTAMP);'
		)
		.run();
	metricsDB.pragma('user_version = 1');
}

if (metricsDB.pragma('user_version', { simple: true }) === 1) {
	metricsDB.prepare('ALTER TABLE metrics ADD referrer TEXT;').run();
	metricsDB.pragma('user_version = 2');
}

const addFeedbackStmt = feedbackDB.prepare(
	'INSERT INTO feedback (content, email) VALUES (@content, @email);'
);

const getFeedbackStmt = feedbackDB.prepare('SELECT * FROM feedback ORDER BY timestamp DESC;');

const addPageViewStmt = metricsDB.prepare(
	'INSERT INTO metrics (page, referrer) VALUES (@page, @referrer);'
);
const getDailyPageViewsStmt = metricsDB.prepare(
	"SELECT COUNT(*) AS count, strftime('%Y-%m-%d', timestamp) day FROM metrics GROUP BY day;"
);
const getMostVisitedStmt = metricsDB.prepare(
	`SELECT page, count(*) AS count FROM metrics WHERE timestamp > datetime('now','-1 month') GROUP BY page ORDER BY count DESC;`
);
const getReferersStmt = metricsDB.prepare(
	`SELECT referrer, count(*) AS count FROM metrics WHERE timestamp > datetime('now','-1 month') GROUP BY referrer ORDER BY count DESC;`
);

export const addFeedback = (content: string, email?: string) => {
	addFeedbackStmt.run({ content, email });
};

export const addPageView = (page: string, referrer: string) => {
	addPageViewStmt.run({ page, referrer });
};

export const getFeedback = async (): Promise<
	Array<{ content: string; email?: string; timestamp: string }>
> => {
	return getFeedbackStmt.all();
};

export const getDailyPageViews = async (): Promise<Array<{ page: string; timestamp: string }>> => {
	return getDailyPageViewsStmt.all();
};

export const getMostVisited = async (): Promise<Array<{ page: string; count: string }>> => {
	return getMostVisitedStmt.all();
};

export const getReferers = async (): Promise<Array<{ referrer: string; count: number }>> => {
	return getReferersStmt.all();
};
