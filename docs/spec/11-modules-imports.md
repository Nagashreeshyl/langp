# Chapter 11 — Modules & Imports

## 11.1 Module System Overview

Lang.P organizes code into **modules** (files) and **packages** (projects). The module system is designed for readability: every dependency is explicitly declared.

## 11.2 Import Syntax

```lp
use module_name.
```

Examples:

```lp
use navigator.
use ai.
use network.
use database.
use filesystem.
use json.
use math.
```

Qualified imports access nested modules:

```lp
use network.http.
use stdlib.collections.list.
```

## 11.3 Import Rules

1. Imports MUST appear at the top of a file, before other declarations (except module declarations).
2. Wildcard imports (`use module.*`) are **forbidden**.
3. Duplicate imports of the same module are a warning.
4. Unused imports are a warning (configurable in linter).

```lp
@ Valid
use json.
use network.

@ Invalid — wildcard
use json.*.

@ Invalid — import after code
x = 10.
use json.
```

## 11.4 Name Resolution

Imported module names are used as prefixes:

```lp
use json.

data = json.parse('{"name": "Naga"}').
text = json.stringify(data).
```

If a name conflicts with a local binding, the local binding takes precedence:

```lp
use json.

function json(),    @ Local function shadows module
    return "custom".
.

print json().    @ Calls local function, not module
```

To disambiguate, use the full module path from the package root.

## 11.5 Module Structure

A standard package layout:

```
my-project/
    lang.toml           @ Package manifest
    main.lp             @ Entry point
    src/
        models/
            user.lp     @ Module: models.user
            post.lp     @ Module: models.post
        services/
            api.lp      @ Module: services.api
    tests/
        test_user.lp
```

## 11.6 Module Declaration

Optional explicit module name:

```lp
module models.user.
```

If omitted, the module name is derived from the file path (see [Chapter 3](03-program-structure.md)).

## 11.7 Re-exports

A module MAY re-export symbols from its dependencies:

```lp
@ In models/__init__.lp (future barrel module support)
use models.user.
use models.post.

@ Re-export (v0.2)
export models.user.User.
export models.post.Post.
```

In v0.1, consumers import submodules directly.

## 11.8 Standard Library Modules

Core stdlib modules are available without installation:

| Module | Description |
|--------|-------------|
| `core` | Built-in types, print, assert |
| `math` | Mathematical functions |
| `json` | JSON parsing and serialization |
| `collections` | List, Dictionary, Set utilities |
| `datetime` | Date and time |
| `filesystem` | File I/O |
| `network` | HTTP, TCP, WebSocket |
| `database` | Database connectivity |
| `crypto` | Cryptographic functions |
| `compression` | Compression algorithms |
| `terminal` | Terminal UI and colors |
| `testing` | Test framework |
| `env` | Environment variables |
| `reflect` | Runtime type reflection |
| `async` | Async runtime primitives |

Framework modules require explicit import:

| Module | Description |
|--------|-------------|
| `navigator` | Browser/desktop UI framework |
| `ai` | AI/LLM framework |

## 11.9 Third-Party Packages

Installed via the `lang` package manager:

```lp
use requests.      @ From lang.toml dependency
use my_lib.utils.
```

See [Chapter 20](20-package-system.md).

## 11.10 Circular Dependencies

Circular module dependencies are **forbidden**. The compiler MUST detect and reject cycles:

```
error[E0501]: circular dependency detected
  models.user → services.api → models.user
```

Resolution strategies:

- Extract shared types into a common module
- Use interfaces to invert dependencies
- Restructure package layout

## 11.11 Conditional Imports

Not supported in v0.1. All imports are static and resolved at compile time.

## 11.12 Module Initialization

Module-level code executes on first import, in dependency order:

```lp
@ In database.lp
print "Initializing database module".
CONNECTION = create_pool().

@ In main.lp
use database.    @ Prints "Initializing database module"
```

Module initialization is thread-safe and occurs exactly once.
