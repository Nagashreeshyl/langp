# FAQ

## Introduction

Frequently asked questions about Lang.P, the toolchain, and learning path. Answers reflect **v0.2.0** as implemented.

**Why an FAQ:** Quick answers without reading an entire chapter.

**When to read:** When stuck on install, syntax, or "does Lang.P support X?"

---

## Syntax

No Lang.P syntax in this chapter — see [34 — Language Reference](34-language-reference.md).

---

## Examples

### General

**Q: What is Lang.P?**  
A: A readable programming language. Programs use `.lp` files and run with `lang`. See [00 — Preface](00-preface.md).

**Q: Is Lang.P the same as Python?**  
A: No. Lang.P uses `.` for statements, `..` for blocks, `@` for comments, and `with` for string joining. See [03 — Language Basics](03-language-basics.md).

**Q: How do I run a program?**  
A: `lang run file.lp` or `lang check file.lp` first.

**Q: Where is the official syntax defined?**  
A: [Grammar Freeze v1.0](../spec/GRAMMAR-FREEZE-v1.md).

---

### Installation

**Q: Install failed — command not found?**  
A: Reload terminal, check `which lang`, run `lang doctor`. See [01 — Installation](01-installation.md).

**Q: Windows support?**  
A: Yes — use the PowerShell install script in README.

**Q: Do I need Rust installed?**  
A: Only if building from source. End users use the install script.

---

### Language features

**Q: Can I use `+` to join strings?**  
A: No. Use `with`: `"Hello " with name.` See [10 — Strings](10-strings.md).

**Q: Does `throw` work?**  
A: No. Use `try`/`catch`/`finally` and `assert`. See [27 — Error Handling](27-error-handling.md).

**Q: Do classes exist?**  
A: Lang.P uses `type` (Beta), not `class`. See [18 — Types and OOP](18-type-oop.md).

**Q: Can I import my own `.lp` files?**  
A: Project multi-file imports are partial. Built-in `use filesystem.` works. See [25 — Imports](25-imports.md).

**Q: HTTP / web server?**  
A: Not implemented — `network` is a stub. See [37 — Future Roadmap](37-future-roadmap.md).

**Q: AI / chatbot?**  
A: Not implemented — `ai` is a stub. See [35 — Complete Projects](35-complete-projects.md) Mini AI Chat.

---

### Tooling

**Q: Difference between `lang` and `langc`?**  
A: `lang` is the user-facing runner and package manager. `langc` adds `--emit ast` and `--emit tokens` for debugging.

**Q: Does `lang fmt` format code?**  
A: v0.2 runs check validation; full formatter is in progress. See [28 — Package Manager](28-package-manager.md).

**Q: IDE support?**  
A: VS Code / Cursor extension with syntax and diagnostics. LSP is Beta.

---

## Common mistakes

**Q: Why "every statement must end with `.`"?**  
A: You omitted the period. Every executable line needs `.` — see [33 — Common Mistakes](33-common-mistakes.md).

**Q: Why "expected `..`"?**  
A: You closed a block with `.` instead of `..`.

---

## Best practices

**Q: Best first project?**  
A: Calculator ([35 — Complete Projects](35-complete-projects.md)), then Todo App.

**Q: Best learning order?**  
A: Follow this book sequentially from [01 — Installation](01-installation.md).

**Q: How do I know if a feature is stable?**  
A: Check [STATUS.md](../../STATUS.md) and chapter banners (Stable vs Experimental Beta).

---

## Exercises

### Beginner

1. Find the answer to "how to comment" without reading the whole book — which chapter?
2. Run `lang --version` and write the version in your notes.
3. List three differences between Lang.P and Python from the FAQ.
4. Find where HTTP is documented as planned-only.
5. Write one FAQ question you still have; search this book's index.

### Intermediate

1. Answer: "Can I use generics?" — read [14 — Collections](14-collections.md) and STATUS.
2. Explain when to use `lang check` vs `lang run`.
3. Document your install path and `which lang` output.
4. Compare FAQ answers to [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md).
5. Propose one new FAQ entry for GitHub.

### Advanced

1. Write a "FAQ for teachers" — how to use this book in a 8-week course.
2. Identify one FAQ answer that would change in v0.3 — cite ROADMAP.
3. Contribute FAQ improvements via pull request.

---

## Summary

The FAQ covers what Lang.P is, how to install and run it, what works in v0.2, and where to find deeper chapters. When in doubt, verify against [STATUS.md](../../STATUS.md) and [34 — Language Reference](34-language-reference.md).

**Previous:** [35 — Complete Projects](35-complete-projects.md) · **Next:** [37 — Future Roadmap](37-future-roadmap.md)

**See also:** [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md), [36 — FAQ](36-faq.md)
