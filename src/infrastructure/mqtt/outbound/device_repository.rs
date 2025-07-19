use rumqttc::AsyncClient;
use uuid::Uuid;

use crate::{application::ports::outbound::device_repository::{CreateDeviceRepository, DeleteDeviceRepository, DeviceRepositoryError, UpdateDeviceRepository}, domain::device::Device};

pub struct MqttDeviceRepository {
    mqtt_client: AsyncClient,
    device_topic: String
}

impl MqttDeviceRepository {
    pub fn new(mqtt_client: AsyncClient, device_topic: &str, device_topic_response: &str) -> Self {
        mqtt_client.subscribe(device_topic_response, rumqttc::QoS::AtLeastOnce);
        return MqttDeviceRepository { mqtt_client, device_topic: device_topic.to_string() }
    }
}

impl CreateDeviceRepository for MqttDeviceRepository {
    async fn create(&self, _device: &Device) -> Result<(), DeviceRepositoryError> {
        todo!()
    }
}

impl UpdateDeviceRepository for MqttDeviceRepository {
    async fn update(&self, _device: &Device) -> Result<(), DeviceRepositoryError> {
        todo!()
    }
}

impl DeleteDeviceRepository for MqttDeviceRepository {
    async fn delete_by_id(&self, _id: Uuid) -> Result<(), DeviceRepositoryError> {
        todo!()
    }
}