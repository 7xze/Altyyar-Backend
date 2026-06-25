mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod repository;

use std::sync::Arc;

use axum::http::Method;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::middleware::AuthCache;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Arc::new(Config::from_env());
    let pool = db::create_pool(&config.database_url).await;
    db::run_migrations(&pool).await;

    let auth_cache = AuthCache::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers(Any);

    let app = Router::new()
        // Health check
        .route("/api/health", get(health_check))

        // Services
        .route("/api/marketplace/services", get(handlers::services::list_services))
        .route("/api/marketplace/services/search", get(handlers::services::search_services))
        .route("/api/marketplace/services", post(handlers::services::create_service))
        .route("/api/marketplace/services/{id}", get(handlers::services::get_service))
        .route("/api/marketplace/services/{id}", put(handlers::services::update_service))
        .route("/api/marketplace/services/{id}", delete(handlers::services::delete_service))

        // Seller services
        .route("/api/marketplace/seller/services", get(handlers::services::get_my_services))
        .route("/api/marketplace/seller/{seller_id}/services", get(handlers::services::get_seller_services))

        // Cart
        .route("/api/marketplace/cart", get(handlers::cart::get_cart))
        .route("/api/marketplace/cart", post(handlers::cart::add_to_cart))
        .route("/api/marketplace/cart", delete(handlers::cart::clear_cart))
        .route("/api/marketplace/cart/{service_id}", put(handlers::cart::update_cart_item))
        .route("/api/marketplace/cart/{service_id}", delete(handlers::cart::remove_from_cart))

        // Orders
        .route("/api/marketplace/orders", post(handlers::orders::create_order))
        .route("/api/marketplace/orders", get(handlers::orders::get_my_orders))
        .route("/api/marketplace/orders/{order_id}", get(handlers::orders::get_order))

        // Payments
        .route("/api/marketplace/payments/intent", post(handlers::payments::create_payment_intent))
        .route("/api/marketplace/payments/order/{order_id}", get(handlers::payments::get_payment_status))

        // Webhook
        .route("/api/webhooks/moyasar", post(handlers::payments::handle_moyasar_webhook))

        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(pool)
        .with_state(config)
        .with_state(auth_cache);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Tayyar Marketplace API starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "tayyar-marketplace",
        "version": "1.0.0"
    }))
}
