# Minecraft Connector API

A small Rust HTTP API that queries the [Minecraft Java Edition Server List Ping/status protocol](https://wiki.vg/Server_List_Ping) and returns the raw status response from a Minecraft server.

The service exposes a single `/status` endpoint. Given a Minecraft server hostname/IP and an optional port, it opens a TCP connection to that server, sends a Minecraft status handshake and status request, then returns the server's status payload.

## What it does

- Runs an HTTP server using [`actix-web`](https://actix.rs/).
- Accepts Minecraft server connection details through query parameters.
- Connects to the target Minecraft server over TCP.
- Sends a Minecraft status handshake packet.
- Sends a status request packet.
- Returns the server response body as UTF-8 text.

This is useful as a lightweight bridge between HTTP clients and Minecraft server status checks.

## Project structure

```text
src/
├── address.rs        # Address model containing server URL and port
├── buffer.rs         # Helpers for writing Minecraft protocol data types
├── main.rs           # Actix web server and /status endpoint
├── messages.rs       # Minecraft handshake/status request implementation
└── server_status.rs  # Serde models for Minecraft status JSON responses
```

## Requirements

- Rust toolchain with Cargo installed
- Network access to the Minecraft server you want to query

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

## Error handling

If the API cannot connect to the target server or fails while querying it, it returns HTTP `500` with a JSON error response:

```json
{
  "error": "..."
}
```

Common causes include:

- Invalid hostname or IP address
- Closed or incorrect port
- Target server is offline
- Firewall/network restrictions
- Target server does not support the expected Minecraft status protocol

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

The current test suite covers Minecraft protocol buffer helpers, packet construction, and deserialization of Minecraft status response models.

## Implementation notes

- The HTTP server binds to `0.0.0.0:8080` in `src/main.rs`.
- Minecraft status querying uses blocking `std::net::TcpStream` calls.
- The blocking work is wrapped in `actix_web::web::block` so it does not run directly on the Actix worker thread.
- Packet construction helpers in `src/buffer.rs` write Minecraft protocol values such as VarInts, strings, and big-endian shorts.
- `src/server_status.rs` contains Serde models for parsing Minecraft status responses, although the current endpoint returns the raw response body instead of deserializing it into those models.

## Security considerations

This service accepts a user-provided hostname/IP and port, then opens a TCP connection to that destination. If exposed publicly, that behavior can be abused for server-side request forgery style probing of internal networks.

Before deploying publicly, consider adding:

- Authentication or authorization
- Rate limiting
- Request timeouts
- Input validation and allowlists/blocklists
- Restrictions on private/internal IP ranges
- Structured logging and monitoring

## Secrets audit

A manual scan of the current repository did not find obvious committed secrets. Checked areas included:

- Hidden environment-style files such as `.env*`
- Filenames containing `secret` or `key`
- Source/config contents for common tokens, passwords, API keys, private keys, and credential patterns
- Tracked git history for the same common secret patterns

No matches were found at the time this README was written. This is not a substitute for a dedicated secret-scanning tool in CI, but the current repository appears clean from the performed checks.

## License

No license file is currently included. Add a `LICENSE` file before distributing or publishing this project if you want to define reuse terms.
