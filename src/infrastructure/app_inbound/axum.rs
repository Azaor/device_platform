use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

use crate::{
    application::ports::app::{AppInbound, AppOutbound}, infrastructure::http::axum::{
        device_handlers::{
            create_device_handler, delete_device_handler, get_device_by_physical_id, get_device_handler, get_devices_handler, update_device_handler
        },
        device_state_handlers::get_device_state_handler,
        events_handlers::{create_event_handler, get_event_handler},
    }
};

#[derive(Debug)]
pub struct AxumAppInbound;

impl AxumAppInbound {
    pub fn new() -> Self {
        AxumAppInbound {}
    }
}

impl AppInbound for AxumAppInbound {
    async fn start_with_outbound<AS: AppOutbound + 'static>(&self, state: AS) -> Result<(), String> {
        let app = Router::new()
            .route("/devices", post(create_device_handler).get(get_devices_handler))
            .route(
                "/devices/{device_id}",
                get(get_device_handler)
                    .delete(delete_device_handler)
                    .post(update_device_handler),
            )
            .route("/physical/{device_physical_id}", get(get_device_by_physical_id))
            .route("/device_states/{device_id}", get(get_device_state_handler))
            .route(
                "/events/{device_id}",
                post(create_event_handler).get(get_event_handler),
            )
            .with_state(Arc::new(state))
            .layer(TraceLayer::new_for_http());
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
        return Ok(());
    }
}
