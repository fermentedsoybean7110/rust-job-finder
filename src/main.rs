use rust_job_finder::{app, config::Config};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let listener = tokio::net::TcpListener::bind(config.addr)
        .await
        .expect("failed to bind address");
    println!("listening on http://{}", config.addr);
    axum::serve(listener, app()).await.expect("server error");
}
