use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub bcrypt_cost: u32,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/rust_advanced_api".to_string());
        
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;
        
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key".to_string());
        
        let bcrypt_cost = std::env::var("BCRYPT_COST")
            .unwrap_or_else(|_| "12".to_string())
            .parse::<u32>()?;

        Ok(Config {
            database_url,
            port,
            jwt_secret,
            bcrypt_cost,
        })
    }
}