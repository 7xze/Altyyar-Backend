use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub mastodon_api_base: String,
    pub jwt_secret: String,
    pub moyasar_secret_key: String,
    pub moyasar_publishable_key: String,
    pub webhook_secret: String,
    pub environment: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("PORT must be a number"),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            mastodon_api_base: env::var("MASTODON_API_BASE")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "".into()),
            moyasar_secret_key: env::var("MOYASAR_SECRET_KEY")
                .unwrap_or_else(|_| "".into()),
            moyasar_publishable_key: env::var("MOYASAR_PUBLISHABLE_KEY")
                .unwrap_or_else(|_| "".into()),
            webhook_secret: env::var("WEBHOOK_SECRET")
                .unwrap_or_else(|_| "change-me".into()),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".into()),
        }
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
