#[derive(serde::Deserialize)]
pub struct Settings {
	pub database: DatabaseSettings,
	pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
	pub url: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
	let settings = config::Config::builder()
		.add_source(config::File::new("configuration.yml", config::FileFormat::Yaml))
		.build()?;

	settings.try_deserialize::<Settings>()
}