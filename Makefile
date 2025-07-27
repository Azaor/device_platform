.PHONY: run-axum-i-pg-o run-mqtt-i-pg-o run-axum-i-mqtt-o

run-axum-i-pg-o:
	cargo run --no-default-features --features="axum postgres_outbound"

build-axum-i-pg-o:
	cargo build --no-default-features --features="axum postgres_outbound" --release

run-mqtt-i-pg-o:
	cargo run --no-default-features --features="mqtt_inbound postgres_outbound"

build-mqtt-i-pg-o:
	cargo build --no-default-features --features="mqtt_inbound postgres_outbound" --release

run-axum-i-mqtt-o:
	cargo run --no-default-features --features="axum mqtt_server_outbound"

build-axum-i-mqtt-o:
	cargo build --no-default-features --features="axum mqtt_server_outbound" --release

run-egui-i-pg-o:
	cargo run --no-default-features --features="egui_inbound postgres_outbound"

build-egui-i-pg-o:
	cargo build --no-default-features --features="egui_inbound postgres_outbound" --release

run-egui-i-client-o:
	cargo run --no-default-features --features="egui_inbound mqtt_client_outbound"

build-egui-i-client-o:
	cargo build --no-default-features --features="egui_inbound mqtt_client_outbound" --release