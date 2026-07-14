# Lang.P Language Philosophy

**Status:** Official  
**Grammar:** [GRAMMAR-FREEZE-v1.md](GRAMMAR-FREEZE-v1.md)

Lang.P is designed to be the most readable general-purpose programming language — approachable on day one, capable for production systems.

---

## 1. Code should read like instructions

Programs are written for humans first, compilers second:

```lp
@ Ask for the user's age and classify them.
age = input number "Enter your age: ".

if age >= 18,
    print "Adult".
otherwise,
    print "Minor".
..
```

If a sentence sounds wrong when read aloud, the syntax is wrong.

---

## 2. Beginners come first

- First programs use only `print`, `input`, variables, and loops
- Error messages explain what, why, and how to fix
- No implicit truthiness, no `+` for strings, no brace matching
- The IDE teaches punctuation: indent on `,`, dedent on `..`

Advanced features (types, async, networking, AI) **extend** the language; they do not replace the beginner model.

---

## 3. Readability over cleverness

Prefer:

```lp
repeat 5 times as i,
    print i.
..
```

Over dense one-liners that require language-lawyer knowledge.

Explicit keywords (`otherwise if`, `repeat forever`, `wait for`) beat punctuation shortcuts.

---

## 4. One obvious way to solve a problem

| Task | Lang.P way |
|------|------------|
| End a statement | `.` |
| Open a block | `,` |
| Close a block | `..` |
| Concatenate strings | `with` |
| Add numbers | `+` |
| Else branch | `otherwise` |

When two syntaxes compete, remove one (Grammar Freeze v1.0 unified block close to `..` only).

---

## 5. Libraries should provide power

The core language stays small. Capability grows through libraries:

```lp
use navigator.
use ai.
use network.
```

Keywords are for structure; libraries are for domains (UI, HTTP, databases, LLMs).

---

## 6. Syntax should remain minimal

Four lexical conventions carry most programs:

| Symbol | Meaning |
|--------|---------|
| `@` | Comment |
| `.` | Statement end |
| `,` | Block open |
| `..` | Block close |

Resist new punctuation unless it removes larger confusion elsewhere.

---

## 7. AI is first-class

Lang.P targets AI-native applications: assistants, agents, tool calling, streaming. The `ai` module is part of the vision, not a plugin culture afterthought.

Syntax for AI code follows the same rules — no special-case punctuation for "AI mode."

---

## 8. Documentation is part of the language

- [Lang.P Manual](../manual/README.md) — tutorial chapters
- [Language Reference](../guides/LANGUAGE-REFERENCE.md) — what works today
- [GRAMMAR-FREEZE-v1.md](GRAMMAR-FREEZE-v1.md) — frozen syntax
- [KEYWORDS.md](KEYWORDS.md) — reserved words

Documentation that contradicts the grammar is a **bug**, not a stylistic choice.

---

## 9. The IDE teaches the language

Lang Studio / VS Code extension:

- Syntax highlighting for `@`, `,`, `..`, keywords
- Auto-indent after `,`
- De-indent on `..`
- Autocomplete from `editors/langp-manifest.json` (implemented symbols only)
- Squiggles from `lang check`

If the IDE and the manual disagree, fix the disagreement immediately.

---

## 10. Production quality without production hostility

Lang.P aims for Rust/Go-level tooling discipline with Python-level first-hour accessibility:

- `lang check` before `lang run`
- Conformance tests for every syntax form
- Grammar freeze before 1.0 release
- Clear split: **implemented** vs **specification** in docs

---

## Related reading

- [DESIGN-DECISIONS.md](DESIGN-DECISIONS.md)
- [01 — Introduction (manual)](../manual/01-introduction.md)
- [02 — Philosophy (manual)](../manual/02-philosophy.md)
