gen-data:
	cargo run --bin generate-data

test-data:
	cargo run --bin test-generated-data

data: gen-data test-data

test:
	cargo test --all-features --lib -- --nocapture