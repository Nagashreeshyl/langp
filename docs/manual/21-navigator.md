# 21 — Navigator Framework

**Status: Specification — browser/desktop UI not in v0.1**

---

> Navigator is Lang.P's planned browser and desktop UI framework. Examples in `examples/browser.lp` are **aspirational**.

---

## Beginner concept (specification)

```lp
use navigator.

@ Open a window with a page.
browser = Browser().
browser.load("https://example.com").
browser.show().
```

---

## Components (specification)

The specification defines UI elements: windows, buttons, forms, navigation, and Chromium-backed rendering. See [Navigator (spec)](../spec/17-navigator.md).

---

## Advanced (specification)

- Multi-window applications
- Custom components
- Event handlers with `on`

```lp
on button.clicked,
    print "Clicked".
..
```

---

## v0.1

Use terminal programs (`print`, `input`) for all user interaction today.

---

## Next steps

- [22 — AI Framework](22-ai-framework.md)
- [Navigator (spec)](../spec/17-navigator.md)
