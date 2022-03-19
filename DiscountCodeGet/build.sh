rm lambda.zip
cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/discount_code_get bootstrap
zip lambda.zip bootstrap