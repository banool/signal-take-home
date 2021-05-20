FROM rust:1.52

WORKDIR /server

COPY . .

RUN cargo install --path . --root .

EXPOSE 8888

ENV RUST_LOG=debug

ENTRYPOINT ["/server/bin/signal-take-home", "-p", "8888", "--address", "0.0.0.0", "--allowed-providers", "api.giphy.com"]
