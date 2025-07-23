.PHONY: run-axum-i-pg-o run-mqtt-i-pg-o run-axum-i-mqtt-o

run-axum-i-pg-o:
	cargo run --no-default-features --features="axum postgres_outbound"

run-mqtt-i-pg-o:
	cargo run --no-default-features --features="mqtt_inbound postgres_outbound"

run-axum-i-mqtt-o:
	cargo run --no-default-features --features="axum mqtt_outbound"

run egui-i-pg-o:
	cargo run --no-default-features --features="egui_inbound postgres_outbound"

run egui-i-client-o:
	cargo run --no-default-features --features="egui_inbound mqtt_client_outbound"