# Chapter 12 — Events

## 12.1 Overview

Lang.P provides first-class event-driven programming with the `on` keyword. Events enable reactive, declarative code for UI, network, lifecycle, and custom signals.

## 12.2 Event Handler Syntax

```lp
on event_source,
    @ handler body
..
```

Examples:

```lp
on button.clicked,
    print "Button clicked".
..

on Browser.Start,
    print "Browser started".
..

on server.request,
    response = handle(server.request).
    server.respond(response).
..
```

## 12.3 Event Sources

An event source is any expression that produces events:

| Source | Event | Description |
|--------|-------|-------------|
| `button.clicked` | Click | UI button click |
| `Browser.Start` | Lifecycle | Browser initialization |
| `Browser.Close` | Lifecycle | Browser shutdown |
| `server.request` | HTTP | Incoming HTTP request |
| `user.message` | AI | User message in AI chat |
| `timer.elapsed` | Timer | Timer fired |
| `window.resized` | UI | Window size changed |

Event names use dot notation: `object.event` or `Type.Event`.

## 12.4 Event Payload

Events MAY carry data accessible within the handler:

```lp
on server.request,
    path = server.request.path.
    method = server.request.method.
    body = server.request.body.
    print method with " " with path.
..
```

The event source expression provides the context object. In the handler scope, the event source is bound to its current value.

## 12.5 Multiple Handlers

Multiple handlers MAY be registered for the same event:

```lp
on button.clicked,
    print "Handler 1".
..

on button.clicked,
    print "Handler 2".
..
```

Handlers execute in registration order unless priority is specified (v0.2).

## 12.6 Custom Events

Types MAY define custom events:

```lp
type DownloadManager,
    event completed.
    event failed.
    event progress.

    function finish(),
        self.completed.emit(file = self.current_file).
    ..
..

on download.completed,
    print "Downloaded: " with download.completed.file.
..
```

### 12.6.1 Event Declaration

```lp
type MyWidget,
    event clicked(x: Int, y: Int).
    event value_changed(old: Int, new: Int).
..
```

### 12.6.2 Event Emission

```lp
self.clicked.emit(x = 10, y = 20).
```

## 12.7 Event Lifecycle

```
Register handler (on) → Event occurs → Handlers invoked in order → Return
```

Handlers run synchronously by default. Async handlers use `wait for`:

```lp
on button.clicked,
    data = wait for fetch(url).
    update_ui(data).
..
```

## 12.8 Removing Handlers

Handlers are automatically removed when their scope is destroyed. Explicit removal (v0.2):

```lp
handler = on button.clicked,
    print "Clicked".
..

@ Later:
handler.disconnect().
```

In v0.1, use object lifecycle for automatic cleanup.

## 12.9 Event Propagation

UI events support bubbling (v0.2):

```lp
on parent.clicked,
    print "Parent clicked".
..

on child.clicked,
    print "Child clicked".
    @ Event bubbles to parent by default
..
```

Use `event.stop()` to prevent propagation (v0.2).

## 12.10 Navigator Events

Browser and UI events from the Navigator framework:

```lp
on browser.tab_changed,
    print "Active tab: " with browser.tab_changed.url.
..

on browser.navigation,
    print "Navigating to: " with browser.navigation.url.
..

on browser.download_started,
    print "Downloading: " with browser.download_started.filename.
..
```

See [Chapter 17](17-navigator.md) for the complete event catalog.

## 12.11 AI Events

```lp
on user.message,
    reply = assistant.chat(user.message).
    print reply.
..

on assistant.response,
    display(assistant.response.text).
..

on assistant.error,
    print "AI Error: " with assistant.error.message.
..
```

See [Chapter 18](18-ai-framework.md).

## 12.12 Error Handling in Event Handlers

Uncaught errors in event handlers are reported to the runtime error handler. By default, they log and continue (non-fatal). Critical handlers SHOULD use try/catch:

```lp
on server.request,
    try,
        response = handle(server.request).
        server.respond(response).
    catch error,
        server.respond(error_response(500, error.message)).
    ..
..
```

## 12.13 Event Loop Integration

Applications with event handlers enter an event loop automatically:

```lp
@ main.lp
browser = Browser(name = "Nova").

on Browser.Start,
    print "Welcome".
..

@ Event loop runs until browser is closed
```

Explicit event loop control:

```lp
EventLoop.run().          @ Block until all events processed
EventLoop.run_async().    @ Non-blocking
EventLoop.stop().         @ Signal shutdown
```
