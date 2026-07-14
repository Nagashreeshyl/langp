# Chapter 3 — Program Structure

## 3.1 Compilation Units

A Lang.P **compilation unit** is a single `.lp` source file. A **program** is one or more compilation units linked together with the package system (see Chapter 20).

## 3.2 Entry Point

A program MUST designate an entry point. Conventions:

| Entry file | Purpose |
|------------|---------|
| `main.lp` | Default executable entry point |
| `lib.lp` | Library module entry (no `main`) |

An executable program MUST contain a `main.lp` file with top-level statements or a `main` function:

```lp
@ Option A — top-level statements
print "Starting application".
run_app().

@ Option B — main function
function main(),
    print "Starting application".
    run_app().
..

@ langc invokes main() automatically if present
```

Rules:

- If both top-level statements and `main()` exist, top-level statements execute first, then `main()`.
- Library packages MUST NOT define a `main()` function.

## 3.3 Top-Level Declarations

The following MAY appear at the top level of a compilation unit:

- `use` import statements
- `function` definitions
- `type` definitions
- `enum` definitions
- Variable bindings
- Top-level statements (executable code)

```lp
use network.
use json.

API_URL = "https://api.example.com".

type Config,
    host.
    port.
..

function load_config() -> Config,
    @ ...
..

config = load_config().
```

## 3.4 Execution Order

Top-level declarations are **hoisted** for name resolution but executed in source order:

1. All `use` imports are resolved first.
2. Type and function declarations are registered.
3. Top-level variable initializers run in source order.
4. Top-level statements run in source order.
5. `main()` is invoked if present.

## 3.5 Module Boundaries

Each `.lp` file is a **module**. The module name defaults to the file path relative to the package root, with path separators replaced by dots:

```
src/network/http.lp  →  network.http
main.lp              →  main
```

Modules MAY explicitly declare a name:

```lp
module network.http.
```

Explicit module names MUST match the file path convention or a compile warning is emitted.

## 3.6 Visibility

Lang.P has three visibility levels:

| Modifier | Scope | Syntax |
|----------|-------|--------|
| `public` | Exported from module (default for `type` members in public API) | `public name` |
| `internal` | Visible within the package | `internal name` |
| `private` | Visible within the file | `private name` |

If no modifier is specified:

- Top-level functions and types default to `public`.
- Type members default to `public`.
- Module-level variables default to `internal`.

```lp
private helper_cache = {}.

public function fetch(url),
    @ ...
..

type User,
    public name.
    internal id.
    private password_hash.
..
```

## 3.7 Namespaces

Imported modules are accessed via dot notation:

```lp
use json.

data = json.parse('{"name": "Naga"}').
print data.name.
```

Wildcard imports are NOT supported — every import MUST be explicit:

```lp
@ Valid
use json.

@ Invalid — wildcard imports are forbidden
use json.*.
```

This ensures readability: the origin of every name is traceable.

## 3.8 Conditional Compilation

Lang.P supports compile-time conditions via `when` blocks (v0.2 planned). In v0.1, use runtime checks or separate build targets.

## 3.9 Embedded Resources

The `embed` directive (v0.2 planned) will allow embedding files at compile time:

```lp
@ Future syntax
embed logo = "assets/logo.png".
```

In v0.1, use the `filesystem` standard library to read resources at runtime.

## 3.10 Program Lifecycle

```
┌─────────────┐
│   Compile   │  langc: lex → parse → analyze → codegen
└──────┬──────┘
       ▼
┌─────────────┐
│    Link     │  Resolve dependencies via lang package manager
└──────┬──────┘
       ▼
┌─────────────┐
│  Initialize │  Runtime init: GC, stdlib, module loaders
└──────┬──────┘
       ▼
┌─────────────┐
│   Execute   │  Top-level code → main() → event loop (if applicable)
└──────┬──────┘
       ▼
┌─────────────┐
│  Shutdown   │  finally blocks, resource cleanup, GC sweep
└─────────────┘
```

Applications using Navigator or async event loops enter a runtime event loop after initialization. The loop terminates when the application calls `exit()` or all non-daemon tasks complete.
