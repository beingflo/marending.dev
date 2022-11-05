import Database from 'better-sqlite3';

export const db = new Database('metrics.db');
const stmt = db.prepare('CREATE TABLE IF NOT EXISTS views (number INTEGER, text TEXT);');
stmt.run();
