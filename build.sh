rm -rf ./release/ord
CC=x86_64-linux-musl-gcc
cargo build -r --target x86_64-unknown-linux-musl --config target.x86_64-unknown-linux-musl.linker=\"x86_64-unknown-linux-musl-gcc\"
cp ./target/x86_64-unknown-linux-musl/release/ord ./release/