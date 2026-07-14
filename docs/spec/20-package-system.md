# Chapter 20 — Package System

## 20.1 Overview

The `lang` package manager handles dependency resolution, installation, publishing, and project management for Lang.P packages.

## 20.2 Project Manifest

Every Lang.P project has a `lang.toml` manifest:

```toml
[package]
name = "my-browser"
version = "1.0.0"
description = "A custom web browser built with Lang.P"
authors = ["Naga <naga@example.com>"]
license = "MIT"
entry = "main.lp"

[dependencies]
navigator = "1.0"
requests = "2.1"
my-utils = { git = "https://github.com/user/my-utils", branch = "main" }

[dev-dependencies]
testing = "1.0"

[build]
target = "native"
optimization = 2
```

### 20.2.1 Manifest Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Package name (snake_case) |
| `version` | Yes | Semver version |
| `description` | No | Human-readable description |
| `authors` | No | List of authors |
| `license` | No | SPDX license identifier |
| `entry` | No | Entry point file (default: `main.lp`) |
| `dependencies` | No | Runtime dependencies |
| `dev-dependencies` | No | Development/test dependencies |
| `build.target` | No | `native`, `bytecode`, `wasm` |
| `build.optimization` | No | 0-3 (none to aggressive) |

## 20.3 CLI Commands

### 20.3.1 Project Management

```bash
lang init my-project          @ Create new project
lang init --template browser  @ Create from template
lang build                    @ Build the project
lang run                      @ Build and run
lang test                     @ Run tests
lang clean                    @ Remove build artifacts
```

### 20.3.2 Dependency Management

```bash
lang add requests             @ Add dependency
lang add --dev testing        @ Add dev dependency
lang remove requests          @ Remove dependency
lang update                   @ Update all dependencies
lang update requests          @ Update specific dependency
lang install                  @ Install dependencies from lock file
```

### 20.3.3 Publishing

```bash
lang publish                  @ Publish to registry
lang search browser           @ Search packages
lang info navigator           @ Show package info
```

## 20.4 Package Layout

Standard project structure:

```
my-project/
    lang.toml           @ Manifest
    lang.lock           @ Lock file (auto-generated)
    main.lp             @ Entry point
    src/                @ Source modules
        utils.lp
        models/
            user.lp
    tests/              @ Test files
        test_utils.lp
    assets/             @ Static assets
    docs/               @ Documentation
    .lang/              @ Local cache (gitignored)
```

## 20.5 Dependency Resolution

The resolver:

1. Reads `lang.toml` dependencies.
2. Fetches packages from registry, git, or local path.
3. Resolves version constraints (semver).
4. Detects and rejects circular dependencies.
5. Writes resolved versions to `lang.lock`.

Version constraints:

```toml
navigator = "1.0"           @ Exactly 1.0.x (>= 1.0.0, < 1.1.0)
requests = "^2.1"           @ Compatible (>= 2.1.0, < 3.0.0)
my-lib = "~1.2.3"           @ Patch updates (>= 1.2.3, < 1.3.0)
utils = ">=1.0.0"           @ Minimum version
pinned = "=1.5.0"           @ Exact version
```

## 20.6 Lock File

`lang.lock` pins exact versions for reproducible builds:

```toml
[package]
name = "my-browser"
version = "1.0.0"

[[dependencies]]
name = "navigator"
version = "1.0.3"
checksum = "sha256:abc123..."

[[dependencies]]
name = "requests"
version = "2.1.0"
checksum = "sha256:def456..."
```

Lock files MUST be committed to version control.

## 20.7 Package Registry

Packages are published to `registry.langp.dev` (default):

```bash
lang publish
@ Uploads package after validation:
@   - Valid lang.toml
@   - Passes all tests
@   - Version not already published
@   - License specified
```

Private registries:

```bash
lang config set registry "https://my-registry.company.com"
```

## 20.8 Project Templates

Built-in templates:

| Template | Command | Description |
|----------|---------|-------------|
| `default` | `lang init` | Empty project |
| `browser` | `lang init --template browser` | Navigator browser |
| `api` | `lang init --template api` | HTTP API server |
| `agent` | `lang init --template agent` | AI agent |
| `cli` | `lang init --template cli` | CLI tool |
| `library` | `lang init --template library` | Library package |

Custom templates:

```bash
lang init --template https://github.com/user/my-template
```

## 20.9 Workspaces

Monorepo support (v0.2):

```toml
[workspace]
members = ["compiler", "runtime", "stdlib", "lang-studio"]
```

## 20.10 Versioning Policy

Packages follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR** — incompatible API changes
- **MINOR** — backward-compatible new features
- **PATCH** — backward-compatible bug fixes

Pre-release versions: `1.0.0-alpha.1`, `1.0.0-beta.2`, `1.0.0-rc.1`.

## 20.11 Package Naming

- Package names: `snake_case`, lowercase, 3-64 characters.
- Scoped packages (v0.2): `@org/package-name`.
- Names MUST NOT conflict with stdlib module names.

Reserved names: `core`, `stdlib`, `test`, `lang`, `langc`, `navigator`, `ai`.

## 20.12 Local Dependencies

```toml
[dependencies]
my-lib = { path = "../my-lib" }
other = { path = "./vendor/other" }
```

Local dependencies are useful for monorepos and development.

## 20.13 Git Dependencies

```toml
[dependencies]
my-lib = { git = "https://github.com/user/my-lib" }
my-lib = { git = "https://github.com/user/my-lib", branch = "dev" }
my-lib = { git = "https://github.com/user/my-lib", tag = "v1.0.0" }
my-lib = { git = "https://github.com/user/my-lib", rev = "abc123" }
```

## 20.14 Feature Flags

Optional features (v0.2):

```toml
[dependencies]
database = { version = "1.0", features = ["postgres", "sqlite"] }
```

```lp
@ Code conditional on features
when feature("postgres"),
    use database.postgres.
.
```
