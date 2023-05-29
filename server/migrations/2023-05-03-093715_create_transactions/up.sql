CREATE TABLE transactions (
	id INTEGER NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
	created_at TEXT NOT NULL,
    kind TEXT NOT NULL,
	mutation INTEGER NOT NULL,
	recipient_id TEXT,
	FOREIGN KEY (user_id) REFERENCES users (id),
	FOREIGN KEY (recipient_id) REFERENCES users (id)
)