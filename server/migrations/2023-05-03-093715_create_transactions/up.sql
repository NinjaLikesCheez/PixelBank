CREATE TABLE transactions (
	id INTEGER NOT NULL PRIMARY KEY,
    account TEXT NOT NULL,
	created_at TEXT NOT NULL,
    kind TEXT NOT NULL,
	balance REAL NOT NULL,
	role TEXT NOT NULL
)