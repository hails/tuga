FROM rust:1.42.0 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=builder /app/target/release/tuga .
CMD [ "./tuga" ]
