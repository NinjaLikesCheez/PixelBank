CREATE TABLE users (
	id TEXT NOT NULL PRIMARY KEY,
	created_at TEXT NOT NULL,
	username TEXT NOT NULL UNIQUE,
	balance INTEGER NOT NULL,
	role TEXT NOT NULL
)