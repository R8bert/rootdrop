use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
    sync::Arc,
};

use crate::tui::{LogLevel, TuiLogger};

pub struct TuiLogging {
    logger: Arc<TuiLogger>,
}

impl TuiLogging {
    pub fn new(logger: Arc<TuiLogger>) -> Self {
        Self { logger }
    }
}

impl<S, B> Transform<S, ServiceRequest> for TuiLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TuiLoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TuiLoggingMiddleware {
            service: Rc::new(service),
            logger: Arc::clone(&self.logger),
        }))
    }
}

pub struct TuiLoggingMiddleware<S> {
    service: Rc<S>,
    logger: Arc<TuiLogger>,
}

impl<S, B> Service<ServiceRequest> for TuiLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let logger = Arc::clone(&self.logger);
        let method = req.method().to_string();
        let path = req.path().to_string();
        let peer_addr = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let start = std::time::Instant::now();
            let res = fut.await?;
            let duration = start.elapsed();
            let status = res.status();

            let level = if status.is_success() {
                LogLevel::Info
            } else if status.is_client_error() {
                LogLevel::Warn
            } else if status.is_server_error() {
                LogLevel::Error
            } else {
                LogLevel::Debug
            };

            logger.log(
                level,
                format!(
                    "{} {} {} - {} ({:.2}ms)",
                    peer_addr,
                    method,
                    path,
                    status.as_u16(),
                    duration.as_secs_f64() * 1000.0
                ),
                Some("http".to_string()),
            );

            Ok(res)
        })
    }
}
