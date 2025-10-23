use crate::handlers::{api::tables::get_tables, assets::serve_asset, spa::serve_spa};
use crate::sql::DatabaseConfig;
use futures::future::BoxFuture;
use http::{Request, Response};
use http_body::Body;
use pin_project::pin_project;
use std::{
    task::{Context, Poll},
    sync::Arc,
};
use tower::Service;

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
        let path = req.uri().path().to_string();

        // Check if the request is for any DevUI route
        if path.starts_with("/dev/ui") {
            let db_config = self.db_config.clone();

            return Box::pin(async move {
                // Handle static assets
                if path.starts_with("/dev/ui/assets/") {
                    let asset_path = path.strip_prefix("/dev/ui/assets/").unwrap_or(&path).to_string();
                    match serve_asset(asset_path).await {
                        Ok(axum_response) => {
                            let (parts, body) = axum_response.into_parts();
                            let tower_response = Response::from_parts(parts, ResBody::from(body));
                            Ok(tower_response)
                        }
                        Err(status) => {
                            let response = Response::builder()
                                .status(status)
                                .body(ResBody::from("Asset not found".to_string()))
                                .unwrap();
                            Ok(response)
                        }
                    }
                }
                // Handle API endpoints
                else if path == "/dev/ui/api/tables" {
                    match get_tables(db_config).await {
                        Ok(axum_response) => {
                            let (parts, body) = axum_response.into_parts();
                            let tower_response = Response::from_parts(parts, ResBody::from(body));
                            Ok(tower_response)
                        }
                        Err(status) => {
                            let response = Response::builder()
                                .status(status)
                                .header("content-type", "application/json")
                                .body(ResBody::from(r#"{"error": "Failed to fetch tables"}"#.to_string()))
                                .unwrap();
                            Ok(response)
                        }
                    }
                }
                // Serve the React app for all other routes
                else {
                    let axum_response = serve_spa().await;
                    let (parts, body) = axum_response.into_parts();
                    let tower_response = Response::from_parts(parts, ResBody::from(body));
                    Ok(tower_response)
                }
            });
        }

        // For all other requests, pass through to the inner service
        let future = self.inner.call(req);
        Box::pin(async move { future.await })
    }
}
