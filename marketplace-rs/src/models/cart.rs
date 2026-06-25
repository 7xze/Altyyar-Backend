use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartItem {
    pub id: Uuid,
    pub user_id: String,
    pub service_id: Uuid,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub service_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartRequest {
    pub quantity: i32,
}
