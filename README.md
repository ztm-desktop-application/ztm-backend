# ztm-backend

Axum backend for the ZTM desktop application.

## Endpoints

| Method | Path            | Description                          |
| ------ | --------------- | ------------------------------------ |
| GET    | `/health-check` | Returns `200 OK` when the server is up |

## Running locally

Requires Rust 1.85 or newer (the crate uses edition 2024).

```sh
cargo run
curl -i localhost:3000/health-check
```

### Configuration

| Variable   | Default                              | Description                    |
| ---------- | ------------------------------------ | ------------------------------ |
| `APP_ADDR` | `0.0.0.0:3000`                       | Address the server listens on  |
| `RUST_LOG` | `ztm_backend=debug,tower_http=debug` | Log filter (`tracing` syntax)  |

Example:

```sh
APP_ADDR=127.0.0.1:8080 RUST_LOG=info cargo run
```

## Running with Docker

```sh
docker build -t ztm-backend .
docker run --rm -p 3000:3000 ztm-backend
curl -i localhost:3000/health-check
```

The server shuts down gracefully on Ctrl+C or SIGTERM, so `docker stop` lets in-flight requests finish.

## Tests

```sh
cargo test
```
