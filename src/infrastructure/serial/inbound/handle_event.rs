use std::collections::HashMap;

use chrono::Utc;
use tracing::{trace, warn};

use crate::{application::ports::{app::AppOutbound, inbound::event_service::{EventService}}, domain::{device::EventFormat, event::{Event, EventDataValue}}};

pub async fn handle_event<AO: AppOutbound + 'static>(app_outbound: AO, device_id: &str, payload: &str) {

    let timestamp = Utc::now();
    let event_data: Vec<&str> = payload.split(',').collect();
    
    if event_data.is_empty() {
        warn!("Received empty payload for device ID: {}", device_id);
        return;
    }

    let mut values = HashMap::new();
    for data in event_data {
        let parts: Vec<&str> = data.split('=').collect();
        if parts.len() == 2 {
            let value = match EventDataValue::try_from(parts[1].to_string().as_str()) {
                Ok(v) => v,
                Err(e) => {
                    warn!("Invalid data format in payload: {}, error: {:?}", data, e);
                    continue;
                },
            };
            values.insert(parts[0].to_string(), value);
        } else {
            warn!("Invalid data format in payload: {}", data);
        }
    }

    let event = Event::new(device_id.to_string(), &timestamp, values);
    let event_service = app_outbound.get_event_service();
    match event_service.handle_event(event, &EventFormat::Json).await {
        Ok(_) => trace!("Event saved successfully for device ID: {}", device_id),
        Err(e) => warn!("Failed to save event for device ID: {}, error: {:?}", device_id, e),
    }

}