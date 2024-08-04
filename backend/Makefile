.PHONY: i
i:
	cargo install cargo-edit
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenvy
	cargo add uuid --features "serde v4, v7, fast-rng"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	cargo add lazy_static
	cargo add jsonwebtoken
# SQLX-CLI
	cargo install sqlx-cli