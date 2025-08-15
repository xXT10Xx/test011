use rust_advanced_api::{
    auth::{hash_password, verify_password},
    config::Config,
};

#[tokio::test]
async fn test_password_hashing() {
    let password = "test_password_123";
    let hash = hash_password(password, Some(4)).unwrap();
    
    assert!(verify_password(password, &hash).unwrap());
    assert!(!verify_password("wrong_password", &hash).unwrap());
}

#[tokio::test]
async fn test_config_from_env() {
    std::env::set_var("DATABASE_URL", "postgresql://test");
    std::env::set_var("PORT", "8080");
    std::env::set_var("JWT_SECRET", "test_secret");
    std::env::set_var("BCRYPT_COST", "10");
    
    let config = Config::from_env().unwrap();
    
    assert_eq!(config.database_url, "postgresql://test");
    assert_eq!(config.port, 8080);
    assert_eq!(config.jwt_secret, "test_secret");
    assert_eq!(config.bcrypt_cost, 10);
}