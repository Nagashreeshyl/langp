# Examples

Example Lang.P programs demonstrating language features and framework usage.

## Programs

| File | Description |
|------|-------------|
| [`hello.lp`](hello.lp) | Canonical hello world |
| [`browser.lp`](browser.lp) | Minimal Navigator browser |
| [`server.lp`](server.lp) | HTTP server with routing |
| [`agent.lp`](agent.lp) | AI assistant with Groq |

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
- Use `with` for string composition (never `+`)
