use std::time::Duration;

use rumqttc::{mqttbytes, AsyncClient, MqttOptions, Packet};

use crate::{application::ports::app::{App, AppState}, domain::device::{EventFormatError}, infrastructure::mqtt::inbound::event_handler::handle_event};

pub struct MQTTApp {
    event_topic: String,
}

impl MQTTApp {
    pub fn new(event_topic: &str) -> Self {
        MQTTApp {
            event_topic: event_topic.to_string(),
        }
    }
    pub async fn router<AS: AppState+'static>(&self, received: &rumqttc::Publish, state: &AS) -> Result<(), HandlerError> {
        if mqttbytes::matches(&received.topic, &self.event_topic) {
            handle_event(received, state).await
        } else {
            Err(HandlerError::ParsingError("Topic does not match with any handler error".to_string()))
        }
    }
}

impl App for MQTTApp {
    async fn start_with_state<AS: AppState+'static>(&self, state: AS) -> Result<(), String> {
        // Here you would set up the MQTT client and connect to the broker
        // For example, using `rumqttc` or another MQTT client library
        let mut mqttoptions = MqttOptions::new("rumqtt-async", "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Subscribe to topics and handle incoming messages
        client.subscribe(&self.event_topic, rumqttc::QoS::AtMostOnce).await.map_err(|e| e.to_string())?;
        // Set up message handling logic here
        // ...
        while let Ok(notification) = eventloop.poll().await {
            if let rumqttc::Event::Incoming(Packet::Publish(published)) = notification {
                match self.router(&published, &state).await {
                    Ok(_) => println!("Event handled successfully for device: {}", published.topic),
                    Err(HandlerError::ParsingError(err)) => {
                        eprintln!("Error parsing event: {}", err);
                    },
                    Err(HandlerError::InternalError(err)) => {
                        eprintln!("Internal error occurred while handling event: {}", err);
                    },
                }
            }
            
        }
        Ok(())
    }
}

pub enum HandlerError {
    ParsingError(String),
    InternalError(String),
}

impl From<EventFormatError> for HandlerError {
    fn from(err: EventFormatError) -> Self {
        match err {
            EventFormatError::UnsupportedFormat(e) => HandlerError::ParsingError(format!("Invalid event format : {}", e)),
        }
    }
}