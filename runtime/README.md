# Runtime

Core runtime types for the Lang.P interpreter.

## Contents

- `Value` — Int, Float, Bool, String, List, Dict, Function, Null
- `RuntimeError` — span-aware runtime errors (`E0300`–`E0399`)
- Built-in function type (`NativeFunction`)

## Status

Phase 9 complete (interpreter-embedded runtime).

## Usage

```bash
cargo test -p langp-runtime
```
