# Examples

Runnable Lang.P programs (`.lp`). Every example **must** pass `lang check` unless marked interactive.

## Running

```bash
lang run examples/hello.lp
lang check examples/*.lp
```

From a cloned repo before install:

```bash
./target/release-fast/lang run examples/hello.lp
```

## Programs

| File | Status | Description |
|------|--------|-------------|
| [`hello.lp`](hello.lp) | ✅ Stable | Canonical hello world |
| [`loops.lp`](loops.lp) | ✅ Stable | `repeat`, `for`, `while` |
| [`calculator.lp`](calculator.lp) | ⚡ Interactive | Math with `input number` (pipe stdin in CI) |
| [`collections.lp`](collections.lp) | ✅ Stable | List, Dict, Set, Tuple |
| [`oop.lp`](oop.lp) | 🟡 Beta | Types, methods, inheritance |
| [`modules.lp`](modules.lp) | 🟡 Beta | `use filesystem`, `use math` |
| [`filesystem_demo.lp`](filesystem_demo.lp) | ✅ Stable | File I/O and filesystem module |
| [`agent.lp`](agent.lp) | 🟡 Stub | AI module stub (`use ai.`) |
| [`browser.lp`](browser.lp) | 🟡 Stub | Navigator module stub |
| [`server.lp`](server.lp) | 🟡 Stub | Network module stub |
| [`input_demo.lp`](input_demo.lp) | ⚡ Interactive | Requires TTY stdin |

### Interactive example

`input_demo.lp` prompts for input. Run manually:

```bash
lang run examples/input_demo.lp
```

Do not run unattended in CI.

## Writing examples

All examples **must**:

- Follow [Grammar Freeze v1.0](../docs/spec/GRAMMAR-FREEZE-v1.md)
- End every statement with `.`
- Close every block with `..`
- Use `with` for string composition (never `+`)
- Include `@` comments for beginners
- Pass `lang check` before commit

See [STYLE_GUIDE.md](../STYLE_GUIDE.md).
