use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub payment_method: String,
    pub gateway_id: String,
    pub gateway_response: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaymentIntentRequest {
    pub order_id: Uuid,
    pub payment_method: String,
    pub callback_url: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentIntentResponse {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub gateway_url: Option<String>,
    pub gateway_id: String,
}

#[derive(Debug, Deserialize)]
pub struct MoyasarWebhookPayload {
    pub id: String,
    pub r#type: String,
    pub data: MoyasarWebhookData,
    pub created_at: i64,
    pub live: bool,
}

#[derive(Debug, Deserialize)]
pub struct MoyasarWebhookData {
    pub id: String,
    pub r#type: String,
    pub attributes: MoyasarPaymentAttributes,
}

#[derive(Debug, Deserialize)]
pub struct MoyasarPaymentAttributes {
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub description: Option<String>,
    pub source: Option<serde_json::Value>,
}
