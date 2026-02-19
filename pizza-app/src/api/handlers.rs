use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use chrono::Utc;
use uuid::Uuid;

use super::{
  error::ApiError,
  models::{CreatePizzaRequest, HealthResponse, Pizza, UpdatePizzaRequest},
  state::AppState,
};

pub async fn health() -> impl IntoResponse {
  Json(HealthResponse { status: "ok" })
}

pub async fn list_pizzas(State(state): State<AppState>) -> impl IntoResponse {
  let pizzas = state.pizzas.read().await;
  let mut items: Vec<Pizza> = pizzas.values().cloned().collect();
  items.sort_by_key(|p| p.created_at);
  Json(items)
}

pub async fn create_pizza(
  State(state): State<AppState>,
  Json(payload): Json<CreatePizzaRequest>,
) -> impl IntoResponse {
  if payload.name.trim().is_empty() {
    return ApiError::BadRequest("name_required").into_response();
  }

  let now = Utc::now();
  let pizza = Pizza {
    id: Uuid::new_v4(),
    name: payload.name,
    size: payload.size,
    toppings: payload.toppings,
    created_at: now,
    updated_at: now,
  };

  state.pizzas.write().await.insert(pizza.id, pizza.clone());
  (StatusCode::CREATED, Json(pizza)).into_response()
}

pub async fn get_pizza(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
  let pizzas = state.pizzas.read().await;
  let pizza = pizzas.get(&id).cloned().ok_or(ApiError::NotFound)?;
  Ok(Json(pizza))
}

pub async fn update_pizza(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
  Json(payload): Json<UpdatePizzaRequest>,
) -> Result<impl IntoResponse, ApiError> {
  let mut pizzas = state.pizzas.write().await;
  let pizza = pizzas.get_mut(&id).ok_or(ApiError::NotFound)?;

  if let Some(name) = payload.name {
    if name.trim().is_empty() {
      return Err(ApiError::BadRequest("name_required"));
    }
    pizza.name = name;
  }

  if let Some(size) = payload.size {
    pizza.size = size;
  }

  if let Some(toppings) = payload.toppings {
    pizza.toppings = toppings;
  }

  pizza.updated_at = Utc::now();

  Ok(Json(pizza.clone()))
}

pub async fn delete_pizza(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
  let mut pizzas = state.pizzas.write().await;
  if pizzas.remove(&id).is_none() {
    return Err(ApiError::NotFound);
  }
  Ok(StatusCode::NO_CONTENT)
}


