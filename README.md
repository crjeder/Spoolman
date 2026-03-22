<picture>
    <img alt="Icon of a filament spool" src="./assets/spoolman-light-logo.png">
</picture>

<br/>


# Spoolman light

_A lightweight filament tracker for home 3D printing._

Spoolman light is a self-hosted web service for tracking your 3D printer filament spools. It is a simplified fork of [Donkie/Spoolman](https://github.com/Donkie/Spoolman) designed for home use — one or two printers and a shelf of spools — with no database server, no vendor management, and no external integrations required.

## Features

* **Filament & Spool Tracking**: Keep records of filament types and individual spools, including color and price directly on the spool.
* **REST API**: A clean [REST API](https://donkie.github.io/Spoolman/) for reading and updating spool data.
* **Web Client**: Built-in browser UI to view, create, edit, and delete filaments and spools.
* **Simple Storage**: All data stored in a single JSON file — no database server required. Configure the path via `SPOOLMAN_DATA_FILE`.

## Integrations

Any Spoolman-compatible REST API client can connect to this service using its standard API endpoints.

**Web client preview:**
![image](https://github.com/Donkie/Spoolman/assets/2332094/33928d5e-440f-4445-aca9-456c4370ad0d)

## Installation

### Docker (recommended)

```bash
docker build -t spoolman-light .
docker run -p 8000:8000 -v /path/to/data:/data \
  -e SPOOLMAN_DATA_FILE=/data/spoolman.json \
  spoolman-light
```

The web UI is available at `http://localhost:8000`.

### Build from source

Requirements: Rust stable ≥ 1.82, `cargo-leptos`, `wasm32-unknown-unknown` target.

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked

cargo leptos build --release
./target/release/spoolman-server
```

## Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust, Axum, Tokio |
| Frontend | Rust, Leptos (WASM), compiled into single binary |
| Storage | JSON file (`spoolman.json`) — no database |
| Types | `spoolman-types` crate shared by server and client |

The entire application ships as a **single self-contained binary** with no Python runtime, no Node.js, and no external database.

## Configuration

| Variable | Default | Purpose |
|----------|---------|---------|
| `SPOOLMAN_DATA_FILE` | `<platform data dir>/spoolman.json` | Path to JSON data file |
| `SPOOLMAN_HOST` | `0.0.0.0` | Bind host |
| `SPOOLMAN_PORT` | `8000` | Bind port |
| `SPOOLMAN_CORS_ORIGIN` | `FALSE` | CORS allowed origin (`FALSE` = disabled) |
| `SPOOLMAN_BASE_PATH` | `""` | URL base path prefix |
| `SPOOLMAN_DEBUG_MODE` | `FALSE` | Enable debug mode |
| `SPOOLMAN_LOGGING_LEVEL` | `info` | Log level (`trace`/`debug`/`info`/`warn`/`error`) |
| `SPOOLMAN_AUTOMATIC_BACKUP` | `TRUE` | Enable daily backup rotation |
