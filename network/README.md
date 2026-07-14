# Network

HTTP, WebSocket, TCP/UDP, DNS, and URL handling for Lang.P programs.

## Capabilities

- Natural-language HTTP syntax: `get url`, `post url with data`
- HTTP server with event-driven request handling
- WebSocket client and server
- TCP/UDP sockets
- DNS resolution
- URL parsing and building
- TLS/SSL support

## Quick Example

```lp
use network.

response = get "https://api.example.com/data".
server = Server(port = 8080).
```

## Status

Part of the standard library. Implemented in Phase 7–9. See [Chapter 15 — I/O & Network](../docs/spec/15-io-network.md).

## Example

See [`examples/server.lp`](../examples/server.lp).
