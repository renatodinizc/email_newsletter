use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;
use env_logger::Env;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    run(listener, db_connection_pool)?.await
}
