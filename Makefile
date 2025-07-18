.PHONY: build release build-minimal build-cli build-web build-cli-web \
        run-cli run-web clean

run-axum-inbound:
	cargo run --no-default-features --features="axum postgres"

run-mqtt-inbound:
	cargo run --no-default-features --features="mqtt postgres"

