use std::{convert::Infallible, time::Duration};

use axum::{
    extract::{Path, State},
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use futures_util::{Stream, StreamExt};
use rdkafka::Message;
use serde::Serialize;
use tokio_stream::StreamExt as TokioStreamExt;

use crate::services::kafka::{models::ConsumeMessage, router::KafkaServiceState};

#[derive(Serialize)]
pub struct ErrorEvent {
    pub error: String,
}

pub async fn consume(
    State(KafkaServiceState(service)): State<KafkaServiceState>,
    Path((cluster_name, topic_name)): Path<(String, String)>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    match service
        .create_consumer(cluster_name.clone(), topic_name.clone())
        .await
    {
        Ok(consumer) => {
          let stream = async_stream::stream! {
            let mut message_stream = consumer.stream();
            while let Some(result) = TokioStreamExt::next(&mut message_stream).await {
              match result {
                Ok(msg) => {
                  let key = msg.key().map(|k| String::from_utf8_lossy(k).to_string());
                  let value = msg.payload().map(|p| String::from_utf8_lossy(p).to_string());
                  let message = ConsumeMessage { key, value };
                  match serde_json::to_string(&message) {
                    Ok(json) => yield Ok(Event::default().data(json)),
                    Err(e) => {
                        tracing::error!("Failed to serialize message: {:?}", e);
                        let error_event = ErrorEvent {
                            error: format!("Serialization error: {}", e),
                        };
                        match serde_json::to_string(&error_event) {
                            Ok(error_json) => yield Ok(Event::default().data(error_json)),
                            Err(_) => yield Ok(Event::default().data("{\"error\":\"Serialization failed\"}")),
                        }
                    }
                  }
                }
                Err(e) => {
                  tracing::error!("Error consuming message: {:?}", e);
                  let error_event = ErrorEvent {
                      error: format!("Consumption error: {}", e),
                  };
                  match serde_json::to_string(&error_event) {
                      Ok(error_json) => yield Ok(Event::default().data(error_json)),
                      Err(_) => yield Ok(Event::default().data("{\"error\":\"Unknown error occurred\"}")),
                  }
                }
              }
            }
          };
          Sse::new(stream.boxed())
        }
        Err(e) => {
          tracing::error!(
              "Failed to create consumer for cluster {} topic {}: {:?}",
              cluster_name,
              topic_name,
              e
          );
          let error_event = ErrorEvent {
              error: format!("Failed to create consumer: {}", e),
          };
          let error_json = serde_json::to_string(&error_event)
              .unwrap_or_else(|_| "{\"error\":\"Unknown error\"}".to_string());
          let error_stream = tokio_stream::iter(vec![Ok(Event::default().data(error_json))]);
          Sse::new(error_stream.boxed())
        }
    }
    .keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(5))
            .text("keepalive"),
    )
}
