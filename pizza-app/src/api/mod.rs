mod error;
mod handlers;
mod models;
mod state;

pub use state::AppState;

use axum::{routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

/// Build the Axum router for the Pizza API.
pub fn router(state: AppState) -> Router {
  Router::new()
    .route("/health", get(handlers::health))
    .route("/pizzas", get(handlers::list_pizzas).post(handlers::create_pizza))
    .route("/pizzas/:id", get(handlers::get_pizza).patch(handlers::update_pizza).delete(handlers::delete_pizza))
    .with_state(state)
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http())
}


