#[derive(serde::Deserialize)]
pub struct Settings {
	pub database: DatabaseSettings,
	pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
	pub path: String,
}


/// A function that returns a Result containing a Settings struct
///
/// # Example
/// ```
/// let settings = get_configuration().expect("Could not load configuration.");
/// ```
/// # Errors
/// 
/// Will return a ConfigError if the configuration file is not found or if the
/// configuration file is not valid.
pub fn get() -> Result<Settings, config::ConfigError> {
	let settings = config::Config::builder()
		.add_source(config::File::new("configuration.yml", config::FileFormat::Yaml))
		.build()?;

	settings.try_deserialize::<Settings>()
}