FROM rust:1.61.0

RUN rustup component add rustfmt \
 && rustup target add wasm32-unknown-unknown \
 && cargo install cargo-watch trunk \
 && cargo install --locked wasm-bindgen-cli

ENV workdir=/usr/src/app/frontend
WORKDIR $workdir
