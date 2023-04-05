use std::net::TcpListener;
use server::startup;
use server::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	let configuration = get_configuration().expect("Failed to read configuration");
	let address = format!("0.0.0.0:{}", configuration.application_port);
	let listener = TcpListener::bind(address).expect("Failed to bind port: 1337");

	startup::run(listener)?.await
}
