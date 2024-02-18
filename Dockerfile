FROM rust:1.75

WORKDIR /usr/src/rinha

RUN mkdir src; touch src/main.rs

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY src/ ./src/
EXPOSE 8000

RUN cargo build --release

CMD ./target/release/rinha-2024Q1-ntex