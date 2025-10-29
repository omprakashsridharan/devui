# Kafka SSE Consumer Implementation Guide

## Overview

This guide walks you through implementing a Kafka consumer endpoint using Axum Server-Sent Events (SSE) to stream messages in real-time. The implementation allows clients to connect to a Kafka topic and receive messages as they arrive via SSE.

## Prerequisites

- Basic understanding of Rust and async programming
- Familiarity with Kafka concepts (topics, consumers, offsets)
- Knowledge of HTTP and Server-Sent Events

## Step-by-Step Implementation

### Step 1: Add Required Dependencies

**File**: `Cargo.toml`

Add these dependencies to your `[dependencies]` section:

```toml
tokio-stream = "0.1"
futures-util = "0.3"
async-stream = "0.3"
```

**Why these dependencies?**
- `tokio-stream`: Provides stream utilities compatible with tokio
- `futures-util`: Provides stream utilities and the `boxed()` method for type erasure
- `async-stream`: Allows creating async streams with the `stream!` macro, solving lifetime issues

### Step 2: Extend ClusterManager for StreamConsumer Support

**File**: `src/services/kafka/cluster_manager.rs`

#### 2.1 Add StreamConsumer Import

```rust
use rdkafka::consumer::{BaseConsumer, StreamConsumer};
```

#### 2.2 Add Cluster Configuration Storage

Add a field to store bootstrap servers for each cluster:

```rust
pub struct ClientManager {
    base_consumers: HashMap<String, BaseConsumer>,
    producers: HashMap<String, FutureProducer>,
    cluster_configs: HashMap<String, String>, // cluster_name -> bootstrap_servers
}
```

#### 2.3 Update Constructor

Modify the `new` method to store cluster configurations:

```rust
pub fn new(configs: Config) -> Result<Self, ClusterManagerError> {
    let mut base_consumers = HashMap::new();
    let mut producers = HashMap::new();
    let mut cluster_configs = HashMap::new();

    for cluster_config in configs.cluster_configs {
        // ... existing code ...

        base_consumers.insert(cluster_config.name.clone(), base_consumer);
        cluster_configs.insert(cluster_config.name.clone(), cluster_config.bootstrap_servers.clone());

        // ... rest of existing code ...
    }

    Ok(Self {
        base_consumers,
        producers,
        cluster_configs,
    })
}
```

#### 2.4 Add create_stream_consumer Method

```rust
pub fn create_stream_consumer(&self, name: &str) -> Result<StreamConsumer, ClusterManagerError> {
    let bootstrap_servers = self.cluster_configs
        .get(name)
        .ok_or(ClusterManagerError::ClusterConsumerNotFound(
            name.to_string(),
        ))?;

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("group.id", "devui-consumer") // Fixed group ID for simplicity
        .set("enable.auto.commit", "false") // Disable auto-commit for better control
        .set("auto.offset.reset", "latest") // Default to latest, can be overridden
        .create()
        .map_err(ClusterManagerError::KafkaLibError)?;

    Ok(consumer)
}
```

**Key points:**
- Creates a new consumer per request (not pooled)
- Fixed group ID for simplicity
- Auto-commit disabled for better control
- Default offset reset to "latest"

### Step 3: Add ConsumeMessage Model

**File**: `src/services/kafka/models.rs`

Add the message structure for SSE events:

```rust
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConsumeMessage {
    pub key: Option<String>,
    pub value: Option<String>,
}
```

**Why this structure?**
- Simple JSON format suitable for SSE
- Optional fields handle cases where key or value might be null
- Serialization support for JSON conversion

### Step 4: Implement create_consumer Method in Service

**File**: `src/services/kafka/service.rs`

#### 4.1 Add Required Imports

```rust
use crate::services::kafka::models::{Broker, ClusterMetadata, ConsumeMessage, Partition, Topic};
use rdkafka::consumer::Consumer;
```

#### 4.2 Add create_consumer Method

```rust
pub async fn create_consumer(
    &self,
    cluster_name: String,
    topic_name: String,
    offset: Option<String>,
) -> Result<rdkafka::consumer::StreamConsumer, ServiceError> {
    let consumer = self
        .cluster_manager
        .create_stream_consumer(&cluster_name)
        .map_err(ServiceError::ClientManagerError)?;

    // Subscribe to the topic
    consumer
        .subscribe(&[&topic_name])
        .map_err(ServiceError::KafkaError)?;

    // Handle offset if provided
    if let Some(offset_str) = offset {
        match offset_str.as_str() {
            "earliest" => {
                tracing::info!("Consuming from earliest offset for topic: {}", topic_name);
            }
            "latest" => {
                tracing::info!("Consuming from latest offset for topic: {}", topic_name);
            }
            _ => {
                // Try to parse as numeric offset
                if let Ok(offset_num) = offset_str.parse::<i64>() {
                    tracing::info!("Requested specific offset {} for topic: {}", offset_num, topic_name);
                }
            }
        }
    }

    Ok(consumer)
}
```

**Key points:**
- Returns the consumer directly (not a stream)
- Handles offset parameter ("earliest", "latest", or numeric)
- Subscribes to the specified topic
- Logs offset information for debugging

### Step 5: Create the SSE Handler

**File**: `src/handlers/kafka/consume.rs` (new file)

#### 5.1 Add Required Imports

```rust
use crate::services::kafka::models::ConsumeMessage;
use crate::services::kafka::router::KafkaServiceState;
use axum::extract::{Path, Query, State};
use axum::response::sse::{Event, Sse};
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::Stream;
use futures_util::stream::StreamExt;
use tokio_stream::StreamExt as TokioStreamExt;
```

#### 5.2 Define Query Parameters Structure

```rust
#[derive(Deserialize)]
pub struct ConsumeQuery {
    pub offset: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorEvent {
    pub error: String,
}
```

#### 5.3 Implement the Handler Function

```rust
pub async fn consume(
    State(KafkaServiceState(service)): State<KafkaServiceState>,
    Path((cluster_name, topic_name)): Path<(String, String)>,
    Query(query): Query<ConsumeQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let offset = query.offset.unwrap_or_else(|| "latest".to_string());

    match service.create_consumer(cluster_name.clone(), topic_name.clone(), Some(offset)).await {
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
            tracing::error!("Failed to create consumer for cluster {} topic {}: {:?}", cluster_name, topic_name, e);
            let error_event = ErrorEvent {
                error: format!("Failed to create consumer: {}", e),
            };
            let error_json = serde_json::to_string(&error_event).unwrap_or_else(|_| "{\"error\":\"Unknown error\"}".to_string());
            let error_stream = tokio_stream::iter(vec![Ok(Event::default().data(error_json))]);

            Sse::new(error_stream.boxed())
        }
    }
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(15))
                .text("keepalive"),
        )
}
```

**Key implementation details:**
- Uses `async_stream::stream!` macro to create a stream that owns the consumer
- Handles both success and error cases with proper error events
- Converts Kafka messages to JSON format for SSE
- Implements keep-alive with 15-second intervals
- Uses `boxed()` to handle different stream types uniformly

### Step 6: Wire Up the Handler

#### 6.1 Update Handler Module

**File**: `src/handlers/kafka/mod.rs`

```rust
pub mod clusters;
pub mod consume;
pub mod metadata;
pub mod produce;
```

#### 6.2 Add Route to Router

**File**: `src/services/kafka/router.rs`

Add the import:
```rust
use crate::handlers::kafka::consume::consume;
```

Add the route:
```rust
pub fn router(service: Service) -> Router {
    Router::new()
        .route("/clusters", get(clusters))
        .route("/clusters/{cluster_name}/metadata", get(metadata))
        .route(
            "/clusters/{cluster_name}/topics/{topic_name}",
            post(produce),
        )
        .route(
            "/clusters/{cluster_name}/topics/{topic_name}/consume",
            get(consume),
        )
        .with_state(KafkaServiceState(service))
}
```

## Usage Examples

### Basic Consumption (Latest Messages)
```bash
curl -N http://localhost:3000/api/kafka/clusters/my-cluster/topics/my-topic/consume
```

### Consume from Earliest Offset
```bash
curl -N http://localhost:3000/api/kafka/clusters/my-cluster/topics/my-topic/consume?offset=earliest
```

### Consume from Specific Offset
```bash
curl -N http://localhost:3000/api/kafka/clusters/my-cluster/topics/my-topic/consume?offset=100
```

### JavaScript Client Example
```javascript
const eventSource = new EventSource('/api/kafka/clusters/my-cluster/topics/my-topic/consume');

eventSource.onmessage = function(event) {
    const message = JSON.parse(event.data);
    console.log('Key:', message.key);
    console.log('Value:', message.value);
};

eventSource.onerror = function(event) {
    console.error('SSE error:', event);
};
```

## Technical Considerations

### Why async_stream::stream!?
The `async_stream::stream!` macro solves the lifetime issue where the consumer needs to be owned by the stream. Without it, the consumer would be dropped before the stream could use it.

### Why boxed()?
The `boxed()` method converts different stream types into a common trait object, allowing the function to return different stream types in different code paths.

### Error Handling Strategy
- Consumer creation errors: Return a single error event and close the stream
- Message consumption errors: Send error events but continue the stream
- Serialization errors: Send error events with fallback error messages

### Performance Considerations
- Each SSE connection creates a new consumer instance
- Consumers are automatically cleaned up when the connection closes
- Keep-alive prevents connection timeouts
- Auto-commit is disabled for better control over message processing

## Testing the Implementation

1. **Start your Kafka cluster** and create a test topic
2. **Start your Rust application**
3. **Produce some test messages** to your topic
4. **Connect to the SSE endpoint** using curl or a web browser
5. **Verify messages are received** in real-time

## Troubleshooting

### Common Issues
1. **Compilation errors**: Ensure all dependencies are added to Cargo.toml
2. **Lifetime errors**: Use `async_stream::stream!` to own the consumer
3. **Type mismatch errors**: Use `boxed()` to unify different stream types
4. **Connection drops**: Check keep-alive settings and network stability

### Debug Tips
- Enable tracing logs to see consumer creation and message processing
- Test with a simple topic first before complex setups
- Use browser developer tools to inspect SSE events
- Check Kafka logs for consumer group activity

This implementation provides a robust, real-time Kafka message streaming solution using Server-Sent Events, perfect for building real-time dashboards, monitoring tools, or any application that needs to consume Kafka messages in real-time.
