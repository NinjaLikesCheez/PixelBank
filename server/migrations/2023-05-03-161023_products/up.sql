CREATE TABLE products (
	id TEXT NOT NULL PRIMARY KEY,
	name TEXT NOT NULL,
	has_deposit BOOLEAN NOT NULL CHECK(has_deposit >=0),
	price INTEGER NOT NULL CHECK(price >= 0)
)