gen-data:
	cargo run --bin generate-data

test-data:
	cargo run --bin test-generated-data

test-output:
	cargo test --lib -- --nocapture

data: gen-data test-data