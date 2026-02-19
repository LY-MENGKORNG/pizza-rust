use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use super::models::Pizza;

#[derive(Clone, Default)]
pub struct AppState {
  pub pizzas: Arc<RwLock<HashMap<Uuid, Pizza>>>,
}


