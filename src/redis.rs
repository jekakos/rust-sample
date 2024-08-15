use std::sync::Arc;
use once_cell::sync::OnceCell;
use redis::{Client as RedisClient, Connection};
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct AppRedis {
    client: Arc<RedisClient>,
}

impl AppRedis {
    pub async fn new() -> Self {
        dotenv().ok();
        let redis_url = dotenv::var("REDIS_URL").expect("Expected Redis URL");
        let client: RedisClient = RedisClient::open(redis_url).unwrap();
        AppRedis { client: Arc::new(client) }
    }

    pub fn get_client(&self) -> &RedisClient {
        Arc::as_ref(&self.client)
    }
}

static REDIS: OnceCell<AppRedis> = OnceCell::new();

pub async fn init_redis() {
    let redis = AppRedis::new().await;
    REDIS.set(redis).unwrap_or_else(|e| {
        panic!("Failed to initialize redis: {:?}", e);
    });
}

pub fn get_redis_client() -> &'static RedisClient  {
    &REDIS.get().expect("Redis not initialized").get_client()
}

pub fn get_redis_connection() -> Connection {
    let client = get_redis_client();
    client.get_connection().expect("Failed to get Redis connection")
}