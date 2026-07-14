# Standard Library

Core modules available to every Lang.P program without external dependencies.

## Modules

| Module | Description | Spec chapter |
|--------|-------------|--------------|
| `core` | Built-in types, print, assert | Chapter 16 |
| `collections` | List, Dictionary, Set utilities | Chapter 16 |
| `math` | Mathematical functions and constants | Chapter 16 |
| `json` | JSON parsing and serialization | Chapter 15 |
| `datetime` | Date, time, and duration | Chapter 16 |
| `filesystem` | File and directory I/O | Chapter 15 |
| `network` | HTTP client/server, WebSocket, DNS | Chapter 15 |
| `database` | Database connectivity and ORM | Chapter 16 |
| `crypto` | Hashing, HMAC, UUID generation | Chapter 16 |
| `compression` | Gzip and other compression | Chapter 15 |
| `terminal` | Terminal UI, colors, tables | Chapter 16 |
| `testing` | Test framework | Chapter 16 |
| `env` | Environment variables | Chapter 16 |
| `reflect` | Runtime type reflection | Chapter 16 |
| `async` | Async runtime primitives | Chapter 14 |
| `logging` | Structured logging | Chapter 16 |
| `regex` | Regular expressions | Chapter 16 |

## Design Principles

1. APIs read like natural instructions
2. Sensible defaults, progressive disclosure
3. 100% unit test coverage
4. Every public function includes an example

## Status

Implemented alongside Phase 7–9 (interpreter and runtime).
