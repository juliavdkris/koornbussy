use axum::{
    Router,
	routing::get,
	extract::Path
};
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

	let app = Router::new()
		.layer(TraceLayer::new_for_http())
		.route("/", get(hello))
		.route("/:name", get(hello_person));

	let listener = tokio::net::TcpListener::bind((HOST, PORT))
		.await
		.unwrap();
	tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
	axum::serve(listener, app).await.unwrap();
}


async fn hello() -> &'static str {
	"Hello world!"
}

async fn hello_person(Path(name): Path<String>) -> String {
	format!("Hello, {name}!")
}
