use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::order::{Order, OrderItem, OrderWithItems};

pub async fn create_order(
    pool: &PgPool,
    buyer_id: &str,
    notes: &str,
) -> AppResult<OrderWithItems> {
    let cart_items = sqlx::query_as::<_, (Uuid, Uuid, String, i64, i32)>(
        "SELECT ci.service_id, s.id, s.title, s.price, ci.quantity
         FROM cart_items ci
         JOIN marketplace_services s ON s.id = ci.service_id
         WHERE ci.user_id = $1 AND s.is_active = TRUE
         ORDER BY ci.created_at DESC",
    )
    .bind(buyer_id)
    .fetch_all(pool)
    .await?;

    if cart_items.is_empty() {
        return Err(AppError::Validation("Cart is empty".into()));
    }

    let total_price: i64 = cart_items
        .iter()
        .map(|(_, _, _, price, qty)| price * *qty as i64)
        .sum();

    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (buyer_id, total_price, status, notes)
         VALUES ($1, $2, 'PENDING', $3)
         RETURNING *",
    )
    .bind(buyer_id)
    .bind(total_price)
    .bind(notes)
    .fetch_one(pool)
    .await?;

    for (service_id, _, title, price, quantity) in &cart_items {
        sqlx::query(
            "INSERT INTO order_items (order_id, service_id, title, price, quantity)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(order.id)
        .bind(service_id)
        .bind(title)
        .bind(price)
        .bind(quantity)
        .execute(pool)
        .await?;
    }

    sqlx::query("DELETE FROM cart_items WHERE user_id = $1")
        .bind(buyer_id)
        .execute(pool)
        .await?;

    let items = sqlx::query_as::<_, OrderItem>(
        "SELECT * FROM order_items WHERE order_id = $1",
    )
    .bind(order.id)
    .fetch_all(pool)
    .await?;

    Ok(OrderWithItems::from((order, items)))
}

pub async fn get_orders_by_buyer(pool: &PgPool, buyer_id: &str) -> AppResult<Vec<OrderWithItems>> {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE buyer_id = $1 ORDER BY created_at DESC",
    )
    .bind(buyer_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for order in orders {
        let items = sqlx::query_as::<_, OrderItem>(
            "SELECT * FROM order_items WHERE order_id = $1",
        )
        .bind(order.id)
        .fetch_all(pool)
        .await?;

        result.push(OrderWithItems::from((order, items)));
    }

    Ok(result)
}

pub async fn get_order_by_id(pool: &PgPool, order_id: Uuid) -> AppResult<OrderWithItems> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(order_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Order not found".into()))?;

    let items = sqlx::query_as::<_, OrderItem>(
        "SELECT * FROM order_items WHERE order_id = $1",
    )
    .bind(order.id)
    .fetch_all(pool)
    .await?;

    Ok(OrderWithItems::from((order, items)))
}

pub async fn update_order_status(
    pool: &PgPool,
    order_id: Uuid,
    status: &str,
) -> AppResult<Order> {
    let valid_statuses = ["PENDING", "CONFIRMED", "IN_PROGRESS", "COMPLETED", "CANCELLED"];
    if !valid_statuses.contains(&status) {
        return Err(AppError::Validation(format!("Invalid status '{}'", status)));
    }

    let order = sqlx::query_as::<_, Order>(
        "UPDATE orders SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *",
    )
    .bind(status)
    .bind(order_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Order not found".into()))?;

    Ok(order)
}

pub async fn get_orders_by_seller(
    pool: &PgPool,
    seller_id: &str,
) -> AppResult<Vec<OrderWithItems>> {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT DISTINCT o.* FROM orders o
         JOIN order_items oi ON oi.order_id = o.id
         JOIN marketplace_services s ON s.id = oi.service_id
         WHERE s.seller_id = $1
         ORDER BY o.created_at DESC",
    )
    .bind(seller_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for order in orders {
        let items = sqlx::query_as::<_, OrderItem>(
            "SELECT * FROM order_items WHERE order_id = $1",
        )
        .bind(order.id)
        .fetch_all(pool)
        .await?;

        result.push(OrderWithItems::from((order, items)));
    }

    Ok(result)
}
