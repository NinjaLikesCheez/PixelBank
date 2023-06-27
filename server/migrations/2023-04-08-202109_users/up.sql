CREATE TABLE users (
	id TEXT NOT NULL PRIMARY KEY,
	created_at TEXT NOT NULL,
	username TEXT NOT NULL UNIQUE,
	balance INTEGER NOT NULL,
	role TEXT NOT NULL
);

INSERT INTO users (id, created_at, username, balance, role) VALUES ("0", "0", "pixelbank", 0, "admin");