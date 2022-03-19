cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/discount_code_create bootstrap
zip lambda.zip bootstrap