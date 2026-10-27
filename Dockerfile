FROM rust:1.98-slim-trixie AS builder
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
# Optional override; otherwise build.rs derives the version from git history.
ARG APP_VERSION
ENV APP_VERSION=$APP_VERSION
RUN cargo build --release

FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ztm-backend /usr/local/bin/app
EXPOSE 3000
CMD ["app"]
