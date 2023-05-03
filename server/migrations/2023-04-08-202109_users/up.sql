CREATE TABLE users (
	id TEXT NOT NULL PRIMARY KEY,
	account TEXT NOT NULL,
	created_at TEXT NOT NULL,
	kind TEXT NOT NULL,
	mutation REAL NOT NULL,
	recipient TEXT,
)