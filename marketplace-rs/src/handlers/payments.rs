use axum::extract::{Path, State};
use axum::Json;
use base64::Engine;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::json;
use sha2::Sha256;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::middleware::AuthenticatedUser;
use crate::models::payment::{
    CreatePaymentIntentRequest, MoyasarWebhookPayload, PaymentIntentResponse,
};
use crate::repository::{orders, payments};
use std::sync::Arc;

type HmacSha256 = Hmac<Sha256>;

pub async fn create_payment_intent(
    State(pool): State<PgPool>,
    State(config): State<Arc<Config>>,
    user: AuthenticatedUser,
    Json(req): Json<CreatePaymentIntentRequest>,
) -> AppResult<Json<PaymentIntentResponse>> {
    let order = orders::get_order_by_id(&pool, req.order_id).await?;

    if order.buyer_id != user.0.id {
        return Err(AppError::Unauthorized("Not your order".into()));
    }

    if order.status != "PENDING" {
        return Err(AppError::Validation(
            "Order is not in a payable state".into(),
        ));
    }

    let existing = payments::get_payment_by_order(&pool, req.order_id).await;
    if let Ok(ref payment) = existing {
        if payment.status == "PAID" {
            return Err(AppError::Validation("Order already paid".into()));
        }
    }

    let amount_cents = order.total_price;
    let currency = "SAR".to_string();
    let description = format!("Tayyar Marketplace Order #{}", order.id);

    let moyasar_payload = json!({
        "amount": amount_cents,
        "currency": currency,
        "description": description,
        "callback_url": req.callback_url,
        "source": {
            "type": req.payment_method
        }
    });

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let auth = base64::engine::general_purpose::STANDARD
        .encode(format!("{}:", config.moyasar_secret_key));

    let moyasar_resp = client
        .post("https://api.moyasar.com/v1/payments")
        .header("Authorization", format!("Basic {}", auth))
        .json(&moyasar_payload)
        .send()
        .await
        .map_err(|e| AppError::Payment(format!("Payment gateway error: {}", e)))?;

    let moyasar_body: serde_json::Value = moyasar_resp
        .json()
        .await
        .map_err(|e| AppError::Payment(format!("Invalid gateway response: {}", e)))?;

    let gateway_id = moyasar_body["id"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();

    let status = moyasar_body["attributes"]["status"]
        .as_str()
        .unwrap_or("pending")
        .to_string();

    let gateway_url = moyasar_body["source"]["transaction_url"]
        .as_str()
        .map(|s| s.to_string());

    let payment = payments::create_payment(
        &pool,
        req.order_id,
        amount_cents,
        &currency,
        &req.payment_method,
        &gateway_id,
    )
    .await?;

    Ok(Json(PaymentIntentResponse {
        id: payment.id.to_string(),
        amount: amount_cents as f64 / 100.0,
        currency,
        status,
        gateway_url,
        gateway_id,
    }))
}

#[derive(serde::Deserialize)]
pub struct PaymentWebhookQuery {
    signature: Option<String>,
}

pub async fn handle_moyasar_webhook(
    State(pool): State<PgPool>,
    State(config): State<Arc<Config>>,
    body: Json<MoyasarWebhookPayload>,
) -> AppResult<Json<serde_json::Value>> {
    let payload = body.into_inner();

    if payload.r#type != "payment" {
        return Ok(Json(json!({"status": "ignored"})));
    }

    let gateway_id = payload.data.id;
    let payment_status = payload.data.attributes.status;

    let db_status = match payment_status.as_str() {
        "paid" => "PAID",
        "failed" => "FAILED",
        "refunded" => "REFUNDED",
        _ => "PENDING",
    };

    let gateway_response = serde_json::to_value(&payload)
        .unwrap_or_else(|_| json!({"error": "serialization failed"}));

    payments::update_payment_status(&pool, &gateway_id, db_status, gateway_response).await?;

    Ok(Json(json!({"status": "ok"})))
}

pub async fn get_payment_status(
    State(pool): State<PgPool>,
    user: AuthenticatedUser,
    Path(order_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let order = orders::get_order_by_id(&pool, order_id).await?;

    if order.buyer_id != user.0.id {
        let seller_orders = orders::get_orders_by_seller(&pool, &user.0.id).await?;
        if !seller_orders.iter().any(|o| o.id == order_id) {
            return Err(AppError::Unauthorized("Not your order".into()));
        }
    }

    let payment = payments::get_payment_by_order(&pool, order_id).await?;

    Ok(Json(json!({
        "payment_id": payment.id,
        "order_id": payment.order_id,
        "amount": payment.amount as f64 / 100.0,
        "currency": payment.currency,
        "status": payment.status,
        "payment_method": payment.payment_method,
        "gateway_id": payment.gateway_id,
        "created_at": payment.created_at,
    })))
}
