# Chapter 22 — Compatibility & Versioning

## 22.1 Overview

Lang.P uses semantic versioning for the language specification, compiler toolchain, standard library, and package ecosystem. This chapter defines version numbering, compatibility guarantees, deprecation policy, and migration procedures.

## 22.2 Version Numbering

All Lang.P artifacts follow [Semantic Versioning 2.0.0](https://semver.org/):

```
MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
```

| Component | Version scope | Example |
|-----------|---------------|---------|
| Language specification | Independent semver | `0.1.0` |
| Compiler (`langc`) | Matches language minor | `0.1.3` |
| Package manager (`lang`) | Independent semver | `0.1.1` |
| Standard library | Matches language minor | `0.1.0` |
| Language Server | Independent semver | `0.1.2` |
| Lang Studio | Independent semver | `1.0.0` |
| Third-party packages | Independent semver | `2.4.1` |

### 22.2.1 Version Components

- **MAJOR** — incompatible language or API changes requiring source migration.
- **MINOR** — backward-compatible new features, new stdlib modules, new keywords (rare).
- **PATCH** — backward-compatible bug fixes, documentation corrections, performance improvements.

### 22.2.2 Pre-release Identifiers

| Identifier | Meaning |
|------------|---------|
| `-alpha.N` | Early development, unstable |
| `-beta.N` | Feature-complete, testing |
| `-rc.N` | Release candidate |

Example: `0.2.0-beta.1`

## 22.3 Language Specification Versioning

The specification version is declared in `docs/spec/README.md` and propagated to all chapter headers.

Current version: **0.1.0**

### 22.3.1 What Constitutes a Spec Change

| Change type | Version bump | Example |
|-------------|-------------|---------|
| New keyword | MINOR (rare) or MAJOR | Adding `defer` |
| New stdlib module | MINOR | Adding `graphics` module |
| Syntax clarification | PATCH | Clarifying block indentation rules |
| Breaking syntax change | MAJOR | Changing statement terminator |
| New type | MINOR | Adding `Duration` type |
| Semantic change | MAJOR or MINOR | Changing integer division behavior |

### 22.3.2 Specification Amendment Process

1. **Proposal** — Open an amendment document describing the change, rationale, and alternatives considered.
2. **Review** — Evaluate against language philosophy (readability, simplicity, one obvious way).
3. **Impact analysis** — Identify affected chapters, tooling, stdlib, and migration path.
4. **Update** — Modify spec chapters, examples, and conformance tests.
5. **Version bump** — Increment spec version per §22.3.1.
6. **Announcement** — Publish changelog entry with migration guide.

## 22.4 Compatibility Guarantees

### 22.4.1 Within a Major Version

Within the same MAJOR version, Lang.P guarantees:

- Source code written for version `X.Y.0` MUST compile and run correctly on version `X.Y.Z` (any patch).
- Source code written for version `X.0.0` SHOULD compile on version `X.Y.Z` (any minor), except when using newly added features.
- The standard library MUST maintain backward compatibility within a major version.
- Package manifests (`lang.toml`) using semver constraints MUST resolve correctly across patches and minors.

### 22.4.2 Across Major Versions

Major version bumps MAY include:

- Removed keywords or syntax
- Changed default behavior
- Removed stdlib modules or functions
- Changed type system rules

Major migrations MUST include:

- A migration guide in `docs/migrations/vX-to-vY.md`
- Compiler warnings for deprecated features (one minor version before removal)
- Automated migration tool when changes affect > 10% of common patterns

### 22.4.3 Toolchain Compatibility Matrix

| langc version | Spec version | lang version | Minimum lang-lsp |
|---------------|-------------|--------------|------------------|
| 0.1.x | 0.1.0 | 0.1.x | 0.1.0 |
| 0.2.x | 0.2.0 | 0.2.x | 0.2.0 |

The compiler MUST report a warning when the project's `lang.toml` specifies a language version newer than the compiler supports.

## 22.5 Deprecation Policy

### 22.5.1 Timeline

| Stage | Duration | Behavior |
|-------|----------|----------|
| Active | Indefinite | Fully supported, no warnings |
| Deprecated | 1 MINOR version | Compiles with deprecation warning |
| Removed | Next MAJOR version | Compile error with migration hint |

Example timeline for deprecating a function:

```
v0.1.0  — function active
v0.2.0  — function deprecated (warning emitted)
v0.3.0  — function deprecated (warning emitted)
v1.0.0  — function removed (compile error)
```

### 22.5.2 Deprecation Syntax

```lp
@deprecated("Use fetch_v2 instead. Will be removed in v1.0.0.")
function fetch(url),
    return fetch_v2(url).
..
```

The compiler MUST emit:

```
warning[W0100]: deprecated
  --> src/api.lp:5:1
   |
 5 | function fetch(url),
   | ^^^^^^^^^^^^^^^^^^^^
   |
   = note: Use fetch_v2 instead. Will be removed in v1.0.0.
```

### 22.5.3 What Can Be Deprecated

- Functions and methods
- Type fields
- Stdlib modules (entire modules)
- Compiler flags
- Syntax forms (with long deprecation period)

Keywords MUST NOT be deprecated without a MAJOR version bump.

## 22.6 Edition System (Future)

Lang.P MAY adopt an edition system (similar to Rust) for major language evolution without breaking existing code:

```toml
[package]
name = "my-app"
edition = "2026"
```

Editions allow incompatible changes while letting projects opt in at their own pace. Editions are planned for v1.0+ if needed.

## 22.7 Feature Stability Levels

| Level | Label | Guarantee |
|-------|-------|-----------|
| Stable | (none) | Full compatibility within major version |
| Beta | `@beta` | API may change in patches; stabilized in next minor |
| Experimental | `@experimental` | No compatibility guarantee; may be removed anytime |
| Internal | `@internal` | Not part of public API; may change without notice |

Stdlib modules start as `@experimental`, graduate to `@beta`, then become stable.

## 22.8 Changelog

All releases MUST include a changelog following [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
# Changelog

## [0.2.0] - 2026-09-01

### Added
- `match` expression for pattern matching
- `defer` statement for scoped cleanup

### Changed
- Improved type inference for empty collections

### Deprecated
- `fetch()` in favor of `fetch_v2()`

### Removed
- (nothing)

### Fixed
- Fixed off-by-one in range expressions
```

Changelog location: `CHANGELOG.md` at repository root.

## 22.9 Package Version Constraints

Package dependencies use semver constraints in `lang.toml`:

```toml
[dependencies]
navigator = "1.0"       @ >= 1.0.0, < 1.1.0
requests = "^2.1"       @ >= 2.1.0, < 3.0.0
utils = "~1.2.3"        @ >= 1.2.3, < 1.3.0
pinned = "=1.5.0"       @ exactly 1.5.0
any = "*"               @ any version (discouraged)
```

The lock file (`lang.lock`) pins exact resolved versions for reproducible builds.

## 22.10 Language Version Declaration

Projects declare the language version they target:

```toml
[package]
name = "my-app"
version = "1.0.0"
lang-version = "0.1"
```

The compiler uses this to:

- Enable or disable features gated by version
- Emit appropriate warnings for deprecated features
- Select correct stdlib version

If `lang-version` is omitted, the compiler uses the latest stable version.

## 22.11 Backward Compatibility Checklist

Before releasing a new version, verify:

- [ ] All conformance tests pass
- [ ] No breaking changes without MAJOR bump
- [ ] Deprecated features have migration paths documented
- [ ] Changelog is updated
- [ ] Spec version is bumped
- [ ] Examples compile with new version
- [ ] Lock file format is compatible (or migration provided)

## 22.12 Version History

| Version | Date | Highlights |
|---------|------|------------|
| 0.1.0 | 2026-07-14 | Initial language specification |
