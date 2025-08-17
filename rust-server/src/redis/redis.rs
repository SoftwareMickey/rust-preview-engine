use dotenvy::dotenv;
use redis::{
    aio::{MultiplexedConnection, PubSub},
    AsyncCommands, Client, RedisResult,
};
use std::env;
use futures_util::StreamExt;

use crate::redis::models::PreviewConfig;

pub async fn get_redis_connection() -> RedisResult<MultiplexedConnection> {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS URL MUST BE SET");

    let client = Client::open(redis_url)?;
    let con = client.get_multiplexed_async_connection().await?;
    Ok(con)
}

pub async fn redis_publisher(site_name: &str) -> RedisResult<()> {
    let mut redis = get_redis_connection()
        .await
        .expect("FAILED TO GET REDIS PUBLISHER");

    let payload = serde_json::json!({
        "siteName": site_name
    })
    .to_string(); // convert JSON to string

    let publish_result = redis.publish::<_, _, i64>("preview_jobs", payload).await;

    match publish_result {
        Ok(_) => {
            println!("PUBLISHED REDIS MESSAGE...")
        }
        Err(e) => {
            eprintln!("FAILED TO PUBLISH REDIS MESSAGE : {:?}", e)
        }
    }

    Ok(())
}

pub async fn get_redis_subscriber() -> RedisResult<PubSub> {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS URL MUST BE SET");

    let client = Client::open(redis_url)?;
    let subscriber = client.get_async_pubsub().await?;

    Ok(subscriber)
}

pub async fn return_preview_port_to_user() -> String {
    let mut subscriber = match get_redis_subscriber().await {
        Ok(s) => s,
        Err(e) => return format!("Failed to connect subscriber: {:?}", e),
    };

    if let Err(e) = subscriber.subscribe("preview-config").await {
        return format!("Failed to subscribe: {:?}", e);
    }

    let mut subscription_stream = subscriber.on_message();

    // Wait for a message safely
    if let Some(msg) = subscription_stream.next().await {
        match msg.get_payload::<String>() {
            Ok(payload) => {
                match serde_json::from_str::<PreviewConfig>(&payload) {
                    Ok(config) => {
                        println!("Received preview path: {}", config.preview_path);
                        config.preview_path
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        format!("Failed to parse JSON: {}", e)
                    }
                }
            }
            Err(e) => format!("Failed to get payload: {:?}", e),
        }
    } else {
        "No message received from Redis subscription".to_string()
    }
}

// pub async fn return_preview_port_to_user() -> String {
//     let mut subscriber = get_redis_subscriber().await.unwrap();
//     subscriber
//         .subscribe("preview-config")
//         .await
//         .expect("FAILED TO SUBSCRIBE TO PREVIEW CHANNEL");

//     let mut subscription_stream = subscriber.on_message();

//     let msg = subscription_stream.next().await.unwrap();
//     let payload: String = msg.get_payload().unwrap();

//     let path = serde_json::from_str::<PreviewConfig>(&payload);

//     match path {
//             Ok(config) => {
//                 println!("Received preview path: {}", config.preview_path);
//                 // You can now use `config.preview_path` in your app logic
//                  return  config.preview_path;
//             }
//             Err(e) => {
//                 eprintln!("Failed to parse JSON: {}", e);
//                 let error = format!("Failed to parse JSON: {}", e);
//                 return error;
//             }
//     }


// }
