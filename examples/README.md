# Examples

Example Lang.P programs demonstrating language features and framework usage.

## Programs

| File | Description |
|------|-------------|
| [`hello.lp`](hello.lp) | Canonical hello world |
| [`loops.lp`](loops.lp) | Loop examples |
| [`input_demo.lp`](input_demo.lp) | User input types |
| [`calculator.lp`](calculator.lp) | Math: + − × ÷ |
| [`browser.lp`](browser.lp) | Minimal Navigator browser (planned) |
| [`server.lp`](server.lp) | HTTP server (planned) |
| [`agent.lp`](agent.lp) | AI assistant (planned) |

## Running Examples

Once the toolchain is built (Phase 7+):

```bash
langc examples/hello.lp --mode interpret
lang run examples/server.lp
```

## Writing Examples

All examples MUST:

- Use valid Lang.P syntax per the [language specification](../docs/spec/)
- Include `@` comments explaining each section for beginners
- End every statement with `.`
- Close every block with `..` (see [Grammar Freeze v1.0](../docs/spec/GRAMMAR-FREEZE-v1.md))
- Use `with` for string composition (never `+`)
