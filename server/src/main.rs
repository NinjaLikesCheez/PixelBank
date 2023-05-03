use std::fmt::Display;
use std::net::TcpListener;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{RunQueryDsl, SqliteConnection, r2d2, QueryDsl};

use server::startup;
use server::configuration::{get_configuration, get_products, ProductConfiguration};
use server::models::Product;

fn update_item(item: Product, pool: Pool<ConnectionManager<SqliteConnection>>) {
	let mut connection = pool.get().expect("Failed to get connection from pool");

	use server::schema::products::dsl::*;

	diesel::update(products)
		.set::<Product>(item)
		.execute(&mut connection)
		.expect("Failed to update product");
}

fn insert_item(item: Product, pool: Pool<ConnectionManager<SqliteConnection>>) {
	let mut connection = pool.get().expect("Failed to get connection from pool");

	use server::schema::products::dsl::*;

	diesel::insert_into(products)
			.values(&item)
			.execute(&mut connection)
			.expect("Couldn't insert product");
}

fn sync_products(product_configuration: ProductConfiguration, pool: Pool<ConnectionManager<SqliteConnection>>) {
	let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		use server::schema::products::dsl::*;

		for product in product_configuration.products {
			let item = products
				.find(product.id.clone())
				.first::<Product>(&mut connection);

			match item {
				Ok(_) => update_item(product, pool.clone()),
				Err(_) => insert_item(product, pool.clone())
			}
		}
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	let configuration = get_configuration().expect("Failed to read configuration");
	let address = format!("0.0.0.0:{}", configuration.application_port);
	let listener = TcpListener::bind(address).expect("Failed to bind port: 1337");

	let manager = r2d2::ConnectionManager::<SqliteConnection>::new(configuration.database.path);
	let pool = r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to load SQLite database");

	let product_configuration = get_products().expect("Failed to read product configuration");
	sync_products(product_configuration, pool.clone());

	startup::run(listener, pool)?.await
}
