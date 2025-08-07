use std::time::Duration;

use tokio::io;
use tracing::warn;

use crate::{application::ports::app::{AppInbound, AppOutbound}};
use crate::infrastructure::serial::inbound::handle_event::handle_event;
pub struct SerialAppInbound {
    port_name: String,
    baud_rate: u32,
    timeout: Duration,
}

impl SerialAppInbound {
    pub fn new() -> Self {
        let port_name =
            std::env::var(format!("SERIAL_PORT")).expect("Environment var SERIAL_PORT not found");
        let baud_rate = std::env::var(format!("SERIAL_BAUD_RATE"))
            .expect("Environment var SERIAL_BAUD_RATE not found");
        let timeout_str = std::env::var(format!("SERIAL_TIMEOUT"))
            .expect("Environment var SERIAL_TIMEOUT not found");
        let timeout = Duration::from_secs(
            u64::from_str_radix(&timeout_str, 10).expect("Invalid timeout value"),
        );
        SerialAppInbound {
            port_name,
            baud_rate: baud_rate.parse().expect("Invalid baud rate"),
            timeout,
        }
    }
}
impl AppInbound for SerialAppInbound {
    async fn start_with_outbound<AO: AppOutbound + 'static>(
        &self,
        outbound: AO,
    ) -> Result<(), String> {
        let mut port = serialport::new(&self.port_name, self.baud_rate)
            .timeout(self.timeout)
            .open()
            .map_err(|e| format!("Failed to open serial port: {}", e))?;

        let mut buffer: Vec<u8> = vec![0; 1024];
        loop {
            match port.read(buffer.as_mut_slice()) {
                Ok(bytes_read) => {
                    let data = &buffer[..bytes_read];
                    let splitted_data = &data.split(|v| v == &59u8).collect::<Vec<&[u8]>>();
                    let id = match splitted_data.get(0) {
                        Some(id) => String::from_utf8_lossy(id).to_string(),
                        None => {
                            warn!("Received invalid data, no ID found : {:?}", data);
                            continue
                        }
                    };
                    let payload: String = match splitted_data.get(1) {
                        Some(payload) => String::from_utf8_lossy(payload).to_string(),
                        None => {
                            warn!("Received invalid data, no ID found : {:?}", data);
                            continue
                        }
                    };
                    handle_event(outbound.clone(), &id, &payload).await;
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    
                }
                Err(e) => {
                    eprintln!("Erreur: {:?}", e);
                    break;
                }
            }
        }

        return Ok(());
    }
}
