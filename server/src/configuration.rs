use crate::models::Product;

// APPLICATION
#[derive(serde::Deserialize)]
pub struct Settings {
	pub database: DatabaseSettings,
	pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
	pub path: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
	let settings = config::Config::builder()
		.add_source(config::File::new("configuration.yml", config::FileFormat::Yaml))
		.build()?;

	settings.try_deserialize::<Settings>()
}

// PRODUCTS
#[derive(serde::Deserialize)]
pub struct ProductConfiguration {
	pub products: Vec<Product>,
}

pub fn get_products() -> Result<ProductConfiguration, config::ConfigError> {
	let products = config::Config::builder()
		.add_source(config::File::new("products.yml", config::FileFormat::Yaml))
		.build()?;

		products.try_deserialize::<ProductConfiguration>()
}