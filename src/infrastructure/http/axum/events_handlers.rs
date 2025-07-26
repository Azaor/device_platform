use std::{collections::HashMap, sync::Arc, usize};

use axum::{ body, extract::{Path, Request, State}, response::{IntoResponse, Response}, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{application::ports::{app::AppOutbound, inbound::{device_service::DeviceService, device_state_service::DeviceStateService, event_service::EventService}}, domain::event::Event, infrastructure::http::axum::error::ErrorResponse};

pub async fn create_event_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(device_id) : Path<String>,
    r: Request,
) -> Result<Json<EventResponse>, Response> {
    let request_body = r.into_body();
    let body_bytes = match body::to_bytes(request_body, usize::MAX).await {
        Ok(body_bytes) => body_bytes,
        Err(_) => return Err(ErrorResponse { status: 400, message: "Invalid body provided".to_string()}.into_response()),
    };
    let event_service = services.get_event_service();
    let device_service = services.get_device_service();
    let device_state_service = services.get_device_state_service();
    let device_id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => return Err(ErrorResponse { status: 400, message: "Invalid device_id format".to_string() }.into_response()),
    };
    let device = match device_service.get_device(device_id).await {
        Ok(Some(device)) => device,
        Ok(None) => return Err(ErrorResponse { status: 404, message: "Device not found".to_string() }.into_response()),
        Err(err) => return Err(ErrorResponse::from(err).into_response()),
    };

    let event = match Event::new(
        &device,
        &Utc::now(),
        &body_bytes,
    ) {
        Ok(event) => event,
        Err(err) => return Err(ErrorResponse::from(err).into_response()),
    };
    let res = match event_service.handle_event(event.clone(), &device.event_format).await {
        Ok(_) => Json(EventResponse::from(event.clone())),
        Err(err) => return Err(ErrorResponse::from(err).into_response()),
    };
    // Update the device state with the event payload
    match device_state_service.create_device_state(device_id, event.payload).await {
        Ok(_) => Ok(res),
        Err(err) => return Err(ErrorResponse::from(err).into_response()),
    }

}

pub async fn get_event_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(device_id): Path<String>,
) -> Result<Json<Vec<EventResponse>>, Response> {
    let event_service = services.get_event_service();
    let device_id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => return Err(ErrorResponse { status: 400, message: "Invalid device_id format".to_string() }.into_response()),
    };
    
    match event_service.get_events(&device_id).await {
        Ok(events) => {
            let response: Vec<EventResponse> = events.into_iter().map(EventResponse::from).collect();
            Ok(Json(response))
        },
        Err(err) => Err(ErrorResponse::from(err).into_response()),
    }
}

#[derive(Serialize)]
pub struct EventResponse {
    pub id: Uuid,
    pub device_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, Value>,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        let payload = event.payload.into_iter().map(|(k, v)| (k, v.into())).collect();
        EventResponse {
            id: event.id,
            device_id: event.device_id,
            timestamp: event.timestamp,
            payload,
        }
    }
}