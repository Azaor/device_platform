use std::time::Duration;

use rumqttc::{AsyncClient, MqttOptions, Packet};

use crate::{
    application::ports::app::{AppInbound, AppOutbound},
    infrastructure::{
        mqtt::inbound::{
            device_handler::handle_device, device_state_handler::handle_device_state,
            error::HandlerError, event_handler::handle_event,
        },
        utils,
    },
};

pub struct MQTTAppInbound {
    mqtt_url: String,
    mqtt_port: u16,
    event_topic: String,
    device_topic: String,
    device_state_topic: String,
}

impl MQTTAppInbound {
    pub fn new() -> Self {
        let config = utils::load_mqtt_config_from_env().expect("Env var not set");
        MQTTAppInbound {
            mqtt_url: config.mqtt_url.to_string(),
            mqtt_port: config.mqtt_port,
            event_topic: config.event_topic.to_string(),
            device_topic: config.device_topic.to_string(),
            device_state_topic: config.device_state_topic.to_string(),
        }
    }
    pub async fn router<AO: AppOutbound + 'static>(
        &self,
        received: &rumqttc::Publish,
        outbound: &AO,
    ) -> Result<(), HandlerError> {
        if received.topic == self.event_topic {
            handle_event(received, outbound).await
        } else if received.topic == self.device_topic {
            handle_device(received, outbound).await
        } else if received.topic == self.device_state_topic {
            handle_device_state(received, outbound).await
        } else {
            Err(HandlerError::ParsingError(
                "Topic does not match with any handler error".to_string(),
            ))
        }
    }
}

impl AppInbound for MQTTAppInbound {
    async fn start_with_outbound<AO: AppOutbound + 'static>(
        &self,
        outbound: AO,
    ) -> Result<(), String> {
        // Here you would set up the MQTT client and connect to the broker
        // For example, using `rumqttc` or another MQTT client library

        let mut mqttoptions = MqttOptions::new(
            "rumqtt-inbound-async",
            self.mqtt_url.clone(),
            self.mqtt_port,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Subscribe to topics and handle incoming messages
        client
            .subscribe(&self.device_topic, rumqttc::QoS::AtMostOnce)
            .await
            .map_err(|e| e.to_string())?;
        client
            .subscribe(&self.device_state_topic, rumqttc::QoS::AtMostOnce)
            .await
            .map_err(|e| e.to_string())?;
        client
            .subscribe(&self.event_topic, rumqttc::QoS::AtMostOnce)
            .await
            .map_err(|e| e.to_string())?;
        // Set up message handling logic here
        // ...
        while let Ok(notification) = eventloop.poll().await {
            if let rumqttc::Event::Incoming(Packet::Publish(published)) = notification {
                match self.router(&published, &outbound).await {
                    Ok(_) => println!("Event handled successfully for device: {}", published.topic),
                    Err(HandlerError::ParsingError(err)) => {
                        eprintln!("Error parsing event: {}", err);
                    }
                    Err(HandlerError::InternalError(err)) => {
                        eprintln!("Internal error occurred while handling event: {}", err);
                    }
                }
            }
        }
        Ok(())
    }
}
