use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::cart::CartItem;

pub async fn get_cart(pool: &PgPool, user_id: &str) -> AppResult<Vec<CartItem>> {
    let items = sqlx::query_as::<_, CartItem>(
        "SELECT ci.* FROM cart_items ci
         JOIN marketplace_services s ON s.id = ci.service_id
         WHERE ci.user_id = $1 AND s.is_active = TRUE
         ORDER BY ci.created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn add_to_cart(pool: &PgPool, user_id: &str, service_id: Uuid) -> AppResult<CartItem> {
    let service = sqlx::query_scalar::<_, i64>("SELECT 1 FROM marketplace_services WHERE id = $1 AND is_active = TRUE")
        .bind(service_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Service not found or inactive".into()))?;

    let item = sqlx::query_as::<_, CartItem>(
        "INSERT INTO cart_items (user_id, service_id, quantity)
         VALUES ($1, $2, 1)
         ON CONFLICT (user_id, service_id)
         DO UPDATE SET quantity = cart_items.quantity + 1
         RETURNING *",
    )
    .bind(user_id)
    .bind(service_id)
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn update_cart_quantity(
    pool: &PgPool,
    user_id: &str,
    service_id: Uuid,
    quantity: i32,
) -> AppResult<CartItem> {
    if quantity < 1 {
        return Err(AppError::Validation("Quantity must be at least 1".into()));
    }

    let item = sqlx::query_as::<_, CartItem>(
        "UPDATE cart_items SET quantity = $1
         WHERE user_id = $2 AND service_id = $3
         RETURNING *",
    )
    .bind(quantity)
    .bind(user_id)
    .bind(service_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Item not found in cart".into()))?;

    Ok(item)
}

pub async fn remove_from_cart(pool: &PgPool, user_id: &str, service_id: Uuid) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM cart_items WHERE user_id = $1 AND service_id = $2")
        .bind(user_id)
        .bind(service_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Item not found in cart".into()));
    }

    Ok(())
}

pub async fn clear_cart(pool: &PgPool, user_id: &str) -> AppResult<()> {
    sqlx::query("DELETE FROM cart_items WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}
