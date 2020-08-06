use deadpool_redis::{cmd};

#[tokio::main]
async fn main() {
    let cfg = deadpool_redis::Config::from_env("REDIS_URL").unwrap();
    let redis_pool = cfg.create_pool().unwrap();

    let mut redis_conn = redis_pool.get().await.unwrap();

    let count: u32 = cmd("INCR")
        .arg("MY_RUST_COUNTER")
        .query_async(&mut redis_conn)
        .await
        .unwrap();

    println!("current counter: {}", count);
}
