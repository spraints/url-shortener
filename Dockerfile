FROM rust:1.89-trixie AS build

WORKDIR /build
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir -p src; touch src/main.rs; cargo fetch
COPY src src
RUN cargo build --release

FROM debian:trixie-slim AS release
COPY --from=build /build/target/release/url-shortener /bin/url-shortener
VOLUME /etc/url-shortener
CMD [ "/bin/url-shortener", "--config", "/etc/url-shortener/links.yml" ]

