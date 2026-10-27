# ztm-backend

Axum backend for the ZTM desktop application.

## Endpoints

| Method | Path            | Description                                       |
| ------ | --------------- | ------------------------------------------------- |
| GET    | `/health-check` | Returns `200 OK` when the server is up            |
| GET    | `/version`      | Returns the server version as JSON (see below)    |

## Versioning

`GET /version` returns `{"version":"YYYYMMDD.N.0"}`, e.g. `{"version":"20261024.2.0"}`:

- `YYYYMMDD` is the UTC date of the last merge to `main` (the HEAD commit).
- `N` counts the earlier merges to `main` on that same day, starting at 0.

The version is valid semver, so clients can compare versions with any semver library; later merges always compare greater, including on the same day.

```sh
$ curl -s localhost:3000/version
{"version":"20261024.2.0"}
```

It is computed at compile time by `build.rs` from the git history, so builds need the full history for the current day (in CI, e.g. `fetch-depth: 0`). Set `APP_VERSION` at build time to override it (`APP_VERSION=1.2.3 cargo build`, or `docker build --build-arg APP_VERSION=1.2.3 .`).

## Running locally

Requires Rust 1.85 or newer (the crate uses edition 2024).

```sh
cargo run
curl -i localhost:3000/health-check
curl -i localhost:3000/version
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

Build-time variables (read by `build.rs` when compiling, not at runtime):

| Variable      | Default                  | Description                                  |
| ------------- | ------------------------ | -------------------------------------------- |
| `APP_VERSION` | Derived from git history | Overrides the version returned by `/version` |

## Running with Docker

```sh
docker build -t ztm-backend .
docker run --rm -p 3000:3000 ztm-backend
curl -i localhost:3000/health-check
curl -i localhost:3000/version
```

The build copies `.git` into the builder stage so the version can be derived from git history. To set it explicitly, pass `--build-arg APP_VERSION=1.2.3`.

The server shuts down gracefully on Ctrl+C or SIGTERM, so `docker stop` lets in-flight requests finish.

## Tests

```sh
cargo test
```
