use axum::{ Json};

use crate::{
    preview::handlers::receive_preview, 
    redis::{queue::dequeue_handler, redis::{ get_redis_subscriber, redis_publisher}}, types::PagesData };

use tokio::sync::watch::Receiver;
use futures_util::StreamExt;

pub async fn start_worker_handler(
    queue_name: &str,
    rx: Receiver<PagesData>,
) {
    println!("👷 Worker started: '{}'", queue_name);
    println!("DEQUEUING....");

    let mut subscriber = get_redis_subscriber().await.unwrap();
    subscriber.subscribe("job_notify").await.expect("FAILED TO SUBSCRIBE TO QUEUE");

    let mut pubsub_stream = subscriber.on_message();

    while let Some(_msg) = pubsub_stream.next().await {
        println!("🔔 Job notification received.");

        loop {
            match dequeue_handler("sclera:jobs").await {
                Ok(Some(job)) => {
                    println!("⚙️ Processing job: {}", &job.job_type);

                    if job.job_type == "react-code-build" {
                        let latest_data = rx.borrow().clone();
                        receive_preview(Json(latest_data.clone())).await.unwrap();

                        if let Some(site_name) = latest_data.site_name {
                            println!("BUILD DONE FOR: {:?}", site_name);
                            match redis_publisher(&site_name).await {
                                Ok(_) => println!("✅ Publish success"),
                                Err(e) => eprintln!("❌ Publish failed: {:?}", e),
                            }
                        }
                    } else {
                        println!("❓ Unknown job type: {}", job.job_type);
                    }
                }
                Ok(None) => {
                    // No more jobs in the queue
                    println!("📭 Queue empty, waiting for next notification.");
                    break;
                }
                Err(e) => {
                    // This likely means queue is empty; break the loop
                    eprintln!("❌ Failed to dequeue job: {:?}", e);
                    break;
                }
            }
        }
    }
}
