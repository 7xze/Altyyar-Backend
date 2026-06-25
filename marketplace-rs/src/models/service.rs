use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketplaceService {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub price: i64,
    pub images: Vec<String>,
    pub seller_id: String,
    pub seller_name: String,
    pub category: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateServiceRequest {
    pub title: String,
    pub description: String,
    pub price: f64,
    pub images: Vec<String>,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServiceRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub images: Option<Vec<String>>,
    pub category: Option<String>,
}
