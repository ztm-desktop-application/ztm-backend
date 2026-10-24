FROM rust:1.98-slim-trixie AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ztm-backend /usr/local/bin/app
EXPOSE 3000
CMD ["app"]
