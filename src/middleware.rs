use crate::sql::{DatabaseConfig, postgres::PostgresIntrospector, DatabaseIntrospector};
use futures::future::BoxFuture;
use http::{Request, Response, StatusCode};
use http_body::Body;
use pin_project::pin_project;
use std::{
    task::{Context, Poll},
    sync::Arc,
};
use tower::Service;
use std::fs;
use std::path::Path;

/// The DevUI middleware service that intercepts `/dev/ui` requests
#[pin_project]
#[derive(Clone)]
pub struct DevUiService<S> {
    inner: S,
    db_config: Option<Arc<DatabaseConfig>>,
}

impl<S> DevUiService<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            db_config: None,
        }
    }

    pub fn with_database_config(mut self, config: DatabaseConfig) -> Self {
        self.db_config = Some(Arc::new(config));
        self
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for DevUiService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Body + Send + 'static + From<String>,
    ResBody::Data: Send,
    ResBody::Error: Send,
{
    type Response = Response<ResBody>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let path = req.uri().path();

        // Check if the request is for any DevUI route
        if path.starts_with("/dev/ui") {
            let db_config = self.db_config.clone();

            // Handle static assets by reading from filesystem
            if path.starts_with("/dev/ui/assets/") {
                let asset_path = path.strip_prefix("/dev/ui/assets/").unwrap_or(path).to_string();
                let full_path = Path::new("frontend/dist/assets").join(&asset_path);

                return Box::pin(async move {
                    match fs::read_to_string(&full_path) {
                        Ok(content) => {
                            let content_type = if asset_path.ends_with(".js") {
                                "application/javascript"
                            } else if asset_path.ends_with(".css") {
                                "text/css"
                            } else {
                                "application/octet-stream"
                            };

                            let response = Response::builder()
                                .status(StatusCode::OK)
                                .header("content-type", content_type)
                                .header("cache-control", "public, max-age=31536000")
                                .body(ResBody::from(content))
                                .unwrap();
                            Ok(response)
                        }
                        Err(_) => {
                            let response = Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(ResBody::from("Asset not found".to_string()))
                                .unwrap();
                            Ok(response)
                        }
                    }
                });
            }

            // Handle API endpoints for database data
            if path == "/dev/ui/api/tables" {
                let future = async move {
                    if let Some(config) = &db_config {
                        if let Some(postgres_config) = config.postgres() {
                            match PostgresIntrospector::new(postgres_config).await {
                                Ok(introspector) => {
                                    match introspector.get_tables().await {
                                        Ok(tables) => {
                                            let json = serde_json::to_string(&tables).unwrap_or_else(|_| "[]".to_string());
                                            let response = Response::builder()
                                                .status(StatusCode::OK)
                                                .header("content-type", "application/json")
                                                .body(ResBody::from(json))
                                                .unwrap();
                                            Ok(response)
                                        }
                                        Err(_) => {
                                            let response = Response::builder()
                                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                .header("content-type", "application/json")
                                                .body(ResBody::from(r#"{"error": "Failed to fetch tables"}"#.to_string()))
                                                .unwrap();
                                            Ok(response)
                                        }
                                    }
                                }
                                Err(_) => {
                                    let response = Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .header("content-type", "application/json")
                                        .body(ResBody::from(r#"{"error": "Failed to connect to database"}"#.to_string()))
                                        .unwrap();
                                    Ok(response)
                                }
                            }
                        } else {
                            let response = Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .header("content-type", "application/json")
                                .body(ResBody::from(r#"{"error": "No PostgreSQL configuration"}"#.to_string()))
                                .unwrap();
                            Ok(response)
                        }
                    } else {
                        let response = Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .header("content-type", "application/json")
                            .body(ResBody::from(r#"{"error": "No database configuration"}"#.to_string()))
                            .unwrap();
                        Ok(response)
                    }
                };
                return Box::pin(future);
            }

            // Serve the React app for all non-API routes
            let html = include_str!("../frontend/dist/index.html");
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/html; charset=utf-8")
                .body(ResBody::from(html.to_string()))
                .unwrap();

            return Box::pin(async move { Ok(response) });
        }

        // For all other requests, pass through to the inner service
        let future = self.inner.call(req);
        Box::pin(async move { future.await })
    }
}
