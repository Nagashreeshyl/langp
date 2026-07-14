# Navigator

Lang.P's flagship framework for building desktop browsers and rich desktop applications.

## Capabilities

- Full Chrome-like browser with tabs, bookmarks, history, downloads
- Custom themes (light, dark, custom)
- JavaScript execution and CSS injection
- Browser extensions
- Desktop application UI components
- Visual browser designer in Lang Studio

## Quick Example

```lp
use navigator.

browser = Browser(),
    name = "Nova".
    homepage = "https://google.com".
    theme = dark.
..
```

## Status

Phase 11 (not yet implemented). See [Chapter 17 — Navigator Framework](../docs/spec/17-navigator.md).

## Architecture

Navigator wraps a native web rendering engine (Chromium on all platforms) and provides Lang.P APIs for browser chrome, events, and customization.

## Example

See [`examples/browser.lp`](../examples/browser.lp).
