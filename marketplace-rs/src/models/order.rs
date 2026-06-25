use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub buyer_id: String,
    pub total_price: i64,
    pub status: String,
    pub notes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub service_id: Uuid,
    pub title: String,
    pub price: i64,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderWithItems {
    pub id: Uuid,
    pub buyer_id: String,
    pub total_price: i64,
    pub status: String,
    pub notes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub items: Vec<OrderItemWithDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemWithDetails {
    pub service_id: Uuid,
    pub title: String,
    pub price: f64,
    pub quantity: i32,
    pub total: f64,
}

impl From<(Order, Vec<OrderItem>)> for OrderWithItems {
    fn from((order, items): (Order, Vec<OrderItem>)) -> Self {
        let items = items
            .into_iter()
            .map(|item| OrderItemWithDetails {
                service_id: item.service_id,
                title: item.title,
                price: item.price as f64 / 100.0,
                quantity: item.quantity,
                total: (item.price * item.quantity as i64) as f64 / 100.0,
            })
            .collect();

        Self {
            id: order.id,
            buyer_id: order.buyer_id,
            total_price: order.total_price,
            status: order.status,
            notes: order.notes,
            created_at: order.created_at,
            updated_at: order.updated_at,
            items,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub notes: Option<String>,
}
