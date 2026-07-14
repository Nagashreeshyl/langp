@ browser.lp — Minimal Navigator browser example.

@ Import the Navigator framework for building desktop browsers.
use navigator.

@ Create a browser window with custom settings.
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
..

@ Respond when the browser finishes starting.
on Browser.Start,
    print "Welcome to Nova Browser!".
..

@ Log every navigation event.
on browser.navigation,
    print "Navigating to: " with browser.navigation.url.
..

@ The Navigator event loop runs until the browser window is closed.
