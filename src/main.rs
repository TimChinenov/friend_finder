use std::net::TcpListener;
use sqlx::PgPool;
use friend_tracker::startup::run;
use friend_tracker::configuration::get_configuration;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to set subscriber");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}