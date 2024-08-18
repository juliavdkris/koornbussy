mod proxy;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;


const HOST: &str = "localhost";
const PORT: u16 = 8000;


#[tokio::main]
async fn main() {
	let filter_layer = EnvFilter::try_from_default_env()
		.unwrap_or(EnvFilter::new("info"));

	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(filter_layer)
		.init();

	let reverse_proxy = tower::service_fn(proxy::reverse_proxy);

	let app = Router::new()
		.layer(TraceLayer::new_for_http())
		.fallback_service(reverse_proxy);

	let listener = tokio::net::TcpListener::bind((HOST, PORT))
		.await
		.unwrap();
	tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
	axum::serve(listener, app).await.unwrap();
}
