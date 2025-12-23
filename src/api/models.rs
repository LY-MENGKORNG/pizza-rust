use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PizzaSize {
  Small,
  Medium,
  Large,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pizza {
  pub id: Uuid,
  pub name: String,
  pub size: PizzaSize,
  pub toppings: Vec<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePizzaRequest {
  pub name: String,
  pub size: PizzaSize,
  #[serde(default)]
  pub toppings: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePizzaRequest {
  pub name: Option<String>,
  pub size: Option<PizzaSize>,
  pub toppings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
  pub status: &'static str,
}
