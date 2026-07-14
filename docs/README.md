# Documentation

Language specification, guides, and reference material for Lang.P.

## Contents

| Path | Description |
|------|-------------|
| [`guides/HOW-TO-CODE.md`](guides/HOW-TO-CODE.md) | Beginner tutorial — start here |
| [`guides/LANGUAGE-REFERENCE.md`](guides/LANGUAGE-REFERENCE.md) | **All commands & functions implemented in v0.1** |
| [`spec/`](spec/) | Authoritative language specification (22 chapters) |
| [`spec/LANGP-SPEC.md`](spec/LANGP-SPEC.md) | Single-document specification for offline reading |
| [`spec/glossary.md`](spec/glossary.md) | Term definitions |
| [`grammar/`](grammar/) | Formal EBNF grammar (Phase 2) |
| [`grammar/LANGP-EBNF.md`](grammar/LANGP-EBNF.md) | Combined grammar reference |

## Specification

The language specification is the foundation of the Lang.P project. All compiler, runtime, stdlib, and tooling implementations MUST conform to it.

- **Version:** 0.1.0
- **Status:** Phase 1 complete
- **Grammar:** Phase 2 complete — [`grammar/README.md`](grammar/README.md)
- **Index:** [`spec/README.md`](spec/README.md)

## Future Additions

| Path | Phase | Description |
|------|-------|-------------|
| `guides/` | 7+ | Tutorials and how-to guides (**HOW-TO-CODE**, **LANGUAGE-REFERENCE**) |
| `grammar/` | 2 | Formal EBNF grammar (**complete**) |
| `migrations/` | 1.0+ | Version migration guides |
| `api/` | 7+ | Standard library API reference |

## Contributing to Docs

Spec changes follow the amendment process in [Chapter 22](spec/22-compatibility-versioning.md):

1. Propose change with rationale
2. Update affected chapters
3. Bump spec version
4. Add conformance tests
