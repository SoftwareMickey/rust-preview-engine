use std::time::Duration;

use redis::{AsyncCommands, RedisResult};
use tokio::time::sleep;

use crate::redis::{models::Job, redis::{get_redis_connection}};

pub async fn queue_handler(queue_name: &str, job: &Job) -> RedisResult<()> {
    let job_json = serde_json::to_string(job).unwrap(); // * serialize
    println!("📤 Job being pushed: {}", job_json);

    let mut conn = get_redis_connection().await?;
    
    conn.rpush::<_, _, ()>(queue_name, &job_json).await?;
    conn.publish::<_, _, ()>("job_notify", "new_job").await?;
    // println!("🔢 Push result: {}", push_result);

    // let keys: Vec<String> = conn.keys("*").await?;
    // println!("🔑 Keys in Redis: {:?}", keys);


    // let contents: Vec<String> = conn.lrange(queue_name, 0, -1).await.unwrap_or_default();
    // println!("🧾 Queue state after push: {:?}", contents);

    // Confirm
    // if contents.contains(&job_json) {
    //     println!("✅ JOB VERIFIED IN QUEUE");
    // } else {
    //     println!("❌ JOB NOT FOUND IN QUEUE");
    // }


    // match push_result {
    //     Ok(_) => println!("✅ JOB PUSHED TO QUEUE..."),
    //     Err(e) => println!("❌ FAILED TO PUSH TO QUEUE...{:?}", e),
    // }

    // let len = conn.llen(queue_name).await?;
    // println!("📦 Job pushed. Queue now has {:?} jobs", len);


    Ok(())
}


pub async fn dequeue_handler(queue_name: &str) -> RedisResult<Option<Job>> {
    let mut conn = get_redis_connection().await?;

    let current_queue: Vec<String> = conn.lrange(queue_name, 0, -1).await.unwrap_or_default();
    println!("🧾 Current Redis queue [{}] state before dequeue: {:?}", queue_name, current_queue);


    for attempt in 0..2 {
        // Pull a job from the queue
        let job_json: Option<String> = conn.lpop(queue_name, None).await?;

        if let Some(json) = job_json {
            let job: Job = serde_json::from_str(&json).map_err(|e| {
                redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Invalid JSON",
                    format!("{:?}", e),
                ))
            })?;
            return Ok(Some(job));
        }

        println!("⏳ Attempt {}: Queue still empty. Retrying...", attempt + 1);
        sleep(Duration::from_millis(100)).await;
    }

    println!("⚠️ Dequeue retries exhausted. No job found.");
    Ok(None)
}