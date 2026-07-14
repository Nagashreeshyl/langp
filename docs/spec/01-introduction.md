# Chapter 1 — Introduction

## 1.1 Purpose

Lang.P (spoken name: **Lang**) is a general-purpose, statically typed programming language with type inference. It is designed for readability first: source code should read like natural instructions while remaining expressive enough for production systems.

This specification defines the syntax, semantics, standard library interfaces, and tooling contracts for Lang.P implementations.

## 1.2 Design Goals

| Goal | Description |
|------|-------------|
| **Readability** | A person with no programming experience should understand most code after one reading. |
| **Simplicity** | One obvious way to accomplish common tasks. |
| **Power** | Full support for OOP, generics, async, events, and systems programming (future). |
| **Toolability** | First-class IDE, LSP, debugger, and formatter support. |
| **Library growth** | Language features grow through libraries, not keyword proliferation. |
| **AI-native** | Built-in AI framework and MCP integration. |

## 1.3 Non-Goals

Lang.P is **not**:

- A domain-specific language (DSL) — it is a complete general-purpose language.
- A macro-heavy metaprogramming language — reflection exists but is constrained.
- A language that sacrifices safety for brevity — static analysis is mandatory.

## 1.4 Target Domains

Lang.P MUST support building:

- CLI tools and automation scripts
- HTTP APIs and web services
- Desktop applications and browsers (via Navigator)
- AI agents and LLM-powered applications
- Games and multimedia (via Graphics/Audio/Video stdlib)
- Mobile apps (future, Phase 15+)
- Operating systems components (future, Phase 20+)

## 1.5 Naming Conventions

| Artifact | Name |
|----------|------|
| Language | Lang.P |
| Spoken name | Lang |
| Source file extension | `.lp` |
| Compiler CLI | `langc` |
| Package manager CLI | `lang` |
| IDE | Lang Studio |
| Language Server | Lang LSP |

## 1.6 Relationship to Other Languages

Lang.P draws inspiration from:

- **Python** — indentation-based blocks, dynamic-feeling syntax with static types
- **Go** — simplicity, one obvious way, tooling-first culture
- **Kotlin** — null safety, extension functions, coroutines-style async
- **Swift** — readable keyword choices, protocol-oriented design
- **TypeScript** — structural typing, gradual adoption of types via inference

Lang.P deliberately avoids:

- C-style braces `{}` for blocks
- Semicolon-terminated statements
- `+` for string concatenation (uses `with` instead)
- `class` keyword (uses `type` instead)
- `//` or `#` comments (uses `@` instead)

## 1.7 Specification Organization

Chapters 2–15 define the core language. Chapters 16–18 define standard library frameworks. Chapters 19–22 define runtime, packaging, tooling, and versioning.

## 1.8 Example Program

The following program demonstrates core Lang.P syntax:

```lp
@ main.lp — A simple HTTP greeting server.

use network.

function handle_request(request),
    name = request.query.get("name", default = "World").
    body = "Hello " with name with "!".
    return response(200, body = body, content_type = "text/plain").
.

server = Server(port = 8080).

on server.request,
    reply = handle_request(server.request).
    server.respond(reply).
.

print "Server running on port 8080".
server.start().
```

## 1.9 Document Conventions

- **Grammar productions** appear in monospace with `|` for alternatives.
- **Keywords** are written in lowercase and are reserved.
- **Placeholders** in examples use angle brackets: `<name>`.
- Line numbers in error messages are 1-indexed.

## 1.10 Versioning

This specification is version **0.1.0**. See [Chapter 22](22-compatibility-versioning.md) for semver policy.
