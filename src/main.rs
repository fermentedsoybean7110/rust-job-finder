use rust_job_finder::{api::JobsClient, app, config::Config};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let listener = tokio::net::TcpListener::bind(config.addr)
        .await
        .expect("failed to bind address");
    println!("listening on http://{}", config.addr);
    let client = JobsClient::new(config.api_base_url);
    axum::serve(listener, app(client))
        .await
        .expect("server error");
}
