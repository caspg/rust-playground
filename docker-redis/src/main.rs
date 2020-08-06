use deadpool_redis::{cmd};

#[tokio::main]
async fn main() {
    // To read config from env variables `features = ["config"]` is required
    // This will read REDIS_URL and other envs
    // https://docs.rs/deadpool-redis/0.6.0/deadpool_redis/struct.Config.html
    let cfg = deadpool_redis::Config::from_env("REDIS").unwrap();
    let redis_pool = cfg.create_pool().unwrap();

    let mut redis_conn = redis_pool.get().await.unwrap();

    let count: u32 = cmd("INCR")
        .arg("MY_RUST_COUNTER")
        .query_async(&mut redis_conn)
        .await
        .unwrap();

    println!("current counter: {}", count);
}
