# Chapter 17 — Navigator Framework

## 17.1 Overview

Navigator is Lang.P's flagship framework for building desktop browsers and rich desktop applications. It enables users to create a complete Chrome-like browser with minimal code.

```lp
use navigator.
```

## 17.2 Design Goals

1. **Beginner-friendly** — a working browser in under 20 lines.
2. **Fully customizable** — every UI element can be configured or replaced.
3. **IDE-integrated** — Lang Studio generates complete browser templates with explanatory comments.
4. **Production-capable** — real Chromium/WebKit rendering engine underneath.

## 17.3 Creating a Browser

### 17.3.1 Minimal Browser

```lp
use navigator.

browser = Browser(),
    name = "Nova".
    homepage = "https://google.com".
.

@ Browser event loop starts automatically
```

### 17.3.2 Full Configuration

```lp
use navigator.

browser = Browser(),
    name = "Nova".
    homepage = "https://google.com".
    theme = dark.
    width = 1400.
    height = 900.
    tabs = enabled.
    bookmarks = enabled.
    history = enabled.
    downloads = enabled.
    devtools = enabled.
    user_agent = "Nova/1.0".
    cache_size = 100.
    javascript = enabled.
    images = enabled.
    cookies = enabled.
.
```

### 17.3.3 Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `name` | String | "Browser" | Window title and app name |
| `homepage` | String | "about:blank" | Default URL on new tab |
| `theme` | Theme | light | `light`, `dark`, or custom |
| `width` | Int | 1200 | Window width in pixels |
| `height` | Int | 800 | Window height in pixels |
| `tabs` | Bool | enabled | Enable tab bar |
| `bookmarks` | Bool | enabled | Enable bookmarks bar |
| `history` | Bool | enabled | Enable browsing history |
| `downloads` | Bool | enabled | Enable download manager |
| `devtools` | Bool | disabled | Enable developer tools |
| `user_agent` | String | auto | Custom user agent string |
| `cache_size` | Int | 50 | Cache size in MB |
| `javascript` | Bool | enabled | Enable JavaScript |
| `images` | Bool | enabled | Load images |
| `cookies` | Bool | enabled | Enable cookies |

## 17.4 Browser Events

```lp
on Browser.Start,
    print "Browser started".
.

on Browser.Close,
    print "Browser closed".
    save_session().
.

on browser.navigation,
    url = browser.navigation.url.
    print "Navigating to: " with url.
.

on browser.tab_changed,
    print "Active tab: " with browser.tab_changed.title.
.

on browser.download_started,
    print "Downloading: " with browser.download_started.filename.
.

on browser.download_completed,
    print "Download complete: " with browser.download_completed.path.
.

on browser.title_changed,
    @ Update window title
.

on browser.page_loaded,
    print "Page loaded: " with browser.page_loaded.url.
.
```

## 17.5 Browser API

```lp
@ Navigation
browser.navigate("https://example.com").
browser.go_back().
browser.go_forward().
browser.reload().
browser.stop().

@ Tabs
tab = browser.new_tab("https://google.com").
browser.close_tab(tab).
browser.switch_tab(tab).
tabs = browser.tabs().

@ JavaScript execution
result = browser.execute_js("document.title").
browser.inject_css("body { background: #1a1a2e; }").

@ Screenshots
browser.screenshot("page.png").
browser.screenshot_region(x = 0, y = 0, width = 800, height = 600, path = "region.png").

@ Bookmarks
browser.add_bookmark("Lang.P", "https://langp.dev").
bookmarks = browser.bookmarks().

@ History
history = browser.history().
browser.clear_history().

@ Downloads
browser.download("https://example.com/file.zip").
downloads = browser.downloads().
```

## 17.6 Custom UI Components

Navigator supports custom UI overlays:

```lp
use navigator.

@ Custom toolbar button
toolbar = browser.toolbar().
button = toolbar.add_button(
    icon = "star".
    tooltip = "Bookmark this page".
).

on button.clicked,
    browser.add_bookmark(browser.current_title, browser.current_url).
.

@ Custom sidebar panel
sidebar = browser.sidebar(width = 300).
sidebar.add_panel(
    title = "Notes".
    content = NotesPanel().
).
```

## 17.7 Themes

Built-in themes:

```lp
browser = Browser(), theme = dark.
browser = Browser(), theme = light.
```

Custom themes:

```lp
my_theme = Theme(),
    background = "#1a1a2e".
    foreground = "#eaeaea".
    accent = "#6c63ff".
    toolbar = "#16213e".
    tab_active = "#0f3460".
    tab_inactive = "#1a1a2e".
    font = "Inter".
    font_size = 14.
.

browser = Browser(), theme = my_theme.
```

## 17.8 Browser Extensions

Extensions add functionality to the browser:

```lp
type AdBlocker extends Extension,
    name = "Ad Blocker".

    on browser.page_loaded,
        browser.execute_js("""
            document.querySelectorAll('[class*="ad"]').forEach(el => el.remove());
        """).
    .
.

browser.install_extension(AdBlocker()).
```

## 17.9 IDE Template Generation

Lang Studio generates a complete browser project:

```
my-browser/
    lang.toml
    main.lp              @ Generated with explanatory comments
    assets/
        icon.png
    themes/
        custom.lp
    extensions/
        adblocker.lp
```

Every generated line includes a beginner-friendly `@` comment explaining its purpose. Comments can be hidden/shown via IDE toggle.

Example generated `main.lp`:

```lp
@ Import the Navigator framework — this gives us browser capabilities.
use navigator.

@ Create a new browser window with custom settings.
browser = Browser(),
    @ The name shown in the window title bar.
    name = "Nova".
    @ The page loaded when the browser first opens.
    homepage = "https://google.com".
    @ Use dark color scheme for the browser UI.
    theme = dark.
    @ Window dimensions in pixels.
    width = 1400.
    height = 900.
    @ Enable the tab bar at the top of the window.
    tabs = enabled.
    @ Enable the bookmarks bar below the address bar.
    bookmarks = enabled.
.

@ This event fires when the browser finishes starting up.
on Browser.Start,
    print "Welcome to Nova Browser!".
.
```

## 17.10 Architecture

```
┌─────────────────────────────────────┐
│           Lang.P Application        │
├─────────────────────────────────────┤
│         Navigator Framework         │
│  ┌─────────┐ ┌────────┐ ┌────────┐ │
│  │ Browser │ │  Tabs  │ │  Theme │ │
│  │ Engine  │ │ Manager│ │ Engine │ │
│  └────┬────┘ └────────┘ └────────┘ │
├───────┼─────────────────────────────┤
│       ▼                             │
│  Native WebView (Chromium/WebKit)   │
├─────────────────────────────────────┤
│         Lang.P Runtime              │
└─────────────────────────────────────┘
```

Navigator wraps a native web rendering engine (Chromium on all platforms, WebKit on macOS as fallback). The Lang.P code controls the browser chrome; the engine handles page rendering.

## 17.11 Desktop Applications

Navigator also supports non-browser desktop apps:

```lp
use navigator.

app = Application(),
    name = "My App".
    width = 800.
    height = 600.
.

window = app.window().
window.add(Button(text = "Click Me")).
window.add(TextLabel(text = "Hello, Lang.P!")).

on app.start,
    window.show().
.
```

UI components: `Button`, `TextLabel`, `TextInput`, `Checkbox`, `Dropdown`, `ListView`, `Canvas`, `MenuBar`, `Dialog`.

## 17.12 Platform Support

| Platform | Status | Engine |
|----------|--------|--------|
| macOS | v1.0 | Chromium / WebKit |
| Windows | v1.0 | Chromium |
| Linux | v1.0 | Chromium |
| Mobile | Future | TBD |
