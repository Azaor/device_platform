# Device Platform
This projects aims to simplify the maintenability of IoT data fetching by using a single project on every part of the stack 

## Usage

Basically, the app works with an inbound layer and an outbound layer. To run the app, you just have to choose the corresponding features (and setup the corresponding ENV VARS) obviously.

The Inbound layers availables are :
- `axum_inbound` which is a simple HTTP API inbound.
- `mqtt_inbound` which is a MQTT inbound, you will have to setup the distant server on to listen
- `egui_inbound` is a gui allowing the user to fetch/update data via the outbound provided [WiP]
- `serial_inbound` is a serial port reader. It will be used to fetch data from physical device. [WiP]

The outbound layers availables are :
- `mqtt_server_outbound` Is a mqtt outbound that send data via mqtt and fetch data from a PostgresDB 
- `mqtt_client_outbound` Is a mqtt outbound that send data via mqtt and fetch data from a HTTP server 
- `postgres_outbound` Is an outbound that save and fetch data from a PostgresDB 
- `in_memory_outbound` Is an outbound that save and fetch data in the memory.

You can also use `make` command for some specific configuration such as:
```sh
make run-axum-i-pg-o
```