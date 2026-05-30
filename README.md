# Minecraft Connector API

A lightweight bridge that lets HTTP clients query Minecraft servers using the [Minecraft Java Edition Server List Ping/status protocol](https://minecraft.wiki/w/Java_Edition_protocol/Server_List_Ping) and receive the raw status response.

The service exposes a single `/status` endpoint. Given a Minecraft server hostname/IP and an optional port, it opens a TCP connection to that server, sends a Minecraft status handshake and status request, then returns the server's status payload.

## What it does

- Runs an HTTP server using [`actix-web`](https://actix.rs/).
- Accepts Minecraft server connection details through query parameters.
- Connects to the target Minecraft server over TCP.
- Sends a Minecraft status handshake packet.
- Sends a status request packet.
- Returns the server response body as UTF-8 text.

## Project structure

```text
src/
├── address.rs        # Address model containing server URL and port
├── buffer.rs         # Helpers for writing Minecraft protocol data types
├── lib.rs            # Library module exports used by the app and tests
├── main.rs           # Actix web server and /status endpoint
├── messages.rs       # Minecraft handshake/status request implementation
└── server_status.rs  # Serde models for Minecraft status JSON responses

tests/
├── buffer_tests.rs         # Protocol buffer helper tests
├── messages_tests.rs       # Packet construction tests
└── server_status_tests.rs  # Status response deserialization tests
```

## Requirements

- Rust toolchain with Cargo installed

Install Rust from <https://rustup.rs/> if needed.

## Running locally

From the project root:

```sh
cargo run
```

The API starts on:

```text
http://0.0.0.0:8080
```

## API usage

### `GET /status`

Queries a Minecraft server and returns its raw status response.

#### Query parameters

| Parameter | Required | Default | Description |
| --- | --- | --- | --- |
| `url` | Yes | None | Minecraft server hostname or IP address |
| `port` | No | `25565` | Minecraft server port |

#### Example: default Minecraft port

```sh
curl "http://localhost:8080/status?url=example.com"
```

#### Example: custom port

```sh
curl "http://localhost:8080/status?url=example.com&port=25565"
```

#### Example response

The response is the Minecraft server's status JSON payload as a raw string. A typical server may return something like:

```json
{
  "version": {
    "name": "1.20.4",
    "protocol": 765
  },
  "players": {
    "max": 20,
    "online": 3,
    "sample": []
  },
  "description": {
    "text": "A Minecraft Server"
  },
  "favicon": "data:image/png;base64,..."
}
```

## Development

### Build

```sh
cargo build
```

### Run

```sh
cargo run
```

### Check formatting

```sh
cargo fmt --check
```

### Run tests

```sh
cargo test
```

The integration test suite in `tests/` covers Minecraft protocol buffer helpers, packet construction, and deserialization of Minecraft status response models.

## License

No license file is currently included. Add a `LICENSE` file before distributing or publishing this project if you want to define reuse terms.
