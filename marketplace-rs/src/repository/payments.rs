use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::payment::Payment;

pub async fn create_payment(
    pool: &PgPool,
    order_id: Uuid,
    amount: i64,
    currency: &str,
    payment_method: &str,
    gateway_id: &str,
) -> AppResult<Payment> {
    let payment = sqlx::query_as::<_, Payment>(
        "INSERT INTO payments (order_id, amount, currency, payment_method, gateway_id)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(order_id)
    .bind(amount)
    .bind(currency)
    .bind(payment_method)
    .bind(gateway_id)
    .fetch_one(pool)
    .await?;

    Ok(payment)
}

pub async fn get_payment_by_order(pool: &PgPool, order_id: Uuid) -> AppResult<Payment> {
    sqlx::query_as::<_, Payment>("SELECT * FROM payments WHERE order_id = $1 ORDER BY created_at DESC LIMIT 1")
        .bind(order_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Payment not found for this order".into()))
}

pub async fn update_payment_status(
    pool: &PgPool,
    gateway_id: &str,
    status: &str,
    gateway_response: serde_json::Value,
) -> AppResult<Payment> {
    let valid_statuses = ["PENDING", "PAID", "FAILED", "REFUNDED"];
    if !valid_statuses.contains(&status) {
        return Err(AppError::Validation(format!("Invalid payment status '{}'", status)));
    }

    let payment = sqlx::query_as::<_, Payment>(
        "UPDATE payments SET status = $1, gateway_response = $2, updated_at = NOW()
         WHERE gateway_id = $3
         RETURNING *",
    )
    .bind(status)
    .bind(&gateway_response)
    .bind(gateway_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Payment not found".into()))?;

    if status == "PAID" {
        sqlx::query("UPDATE orders SET status = 'CONFIRMED', updated_at = NOW() WHERE id = $1")
            .bind(payment.order_id)
            .execute(pool)
            .await?;
    }

    Ok(payment)
}
