use std::{env, net::SocketAddr};

use pizza_app::api::{router, AppState};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "pizza_app=debug,tower_http=debug".into()))
    .with(tracing_subscriber::fmt::layer())
    .init();

  let port: u16 = env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(3000);
  let addr: SocketAddr = ([0, 0, 0, 0], port).into();

  let state = AppState::default();
  let app = router(state);

  tracing::info!("listening on http://{addr}");
  let listener = TcpListener::bind(addr).await?;
  axum::serve(listener, app).await?;

  Ok(())
}
