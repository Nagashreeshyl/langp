# Tests

Test suites for the Lang.P compiler, runtime, standard library, and tooling.

## Test Categories

| Directory | Purpose | Phase |
|-----------|---------|-------|
| `conformance/` | Language specification conformance tests | Phase 2+ |
| `lexer/` | Lexer unit tests | Phase 3 (**complete**) |
| `parser/` | Parser unit tests | Phase 4 |
| `compiler/` | End-to-end compilation tests | Phase 6+ |
| `runtime/` | Runtime and GC tests | Phase 9 |
| `stdlib/` | Standard library unit tests | Phase 7+ |
| `integration/` | Cross-module integration tests | Phase 7+ |

## Conformance Testing

An implementation is **Lang.P 0.1 conformant** if it passes all tests in `conformance/`. Conformance tests are derived directly from **MUST** requirements in the language specification.

## Running Tests

```bash
lang test                  # Run all tests
lang test --filter lexer   # Run specific category
```

## Writing Tests

Lang.P tests use the built-in testing framework:

```lp
use testing.

test "addition works",
    assert add(2, 3) == 5.
.

test "division by zero throws",
    assert_throws(DivisionError, function(),
        divide(1, 0).
    ).
.
```

Every public API MUST have unit tests. Every spec **MUST** requirement MUST have a conformance test.
