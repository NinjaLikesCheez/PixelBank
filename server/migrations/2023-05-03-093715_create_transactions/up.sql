CREATE TABLE transactions (
	id INTEGER NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
	created_at TEXT NOT NULL,
    kind TEXT NOT NULL,
	balance REAL NOT NULL,
	recipient_id TEXT,
	FOREIGN KEY (user) REFERENCES users (id),
	FOREIGN KEY (recipient) REFERENCES users (id)
)