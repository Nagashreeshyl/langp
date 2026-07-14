# Chapter 14 — Concurrency & Async

> **Implementation note (v0.1):** `async`, `wait for`, and concurrency primitives are **specification only**. See [15 — Async Programming (manual)](../manual/15-async.md).

## 14.1 Overview

Lang.P provides async/await for non-blocking operations and primitives for concurrent execution. The primary async syntax uses `wait for`.

## 14.2 Async Functions

Functions that perform async operations are marked with `async`:

```lp
async function fetch_data(url),
    response = wait for get url.
    return response.body.
..
```

Calling an async function without `wait for` returns a `Task<T>`:

```lp
task = fetch_data("https://api.example.com").    @ Returns Task<String>
result = wait for task.                          @ Blocks until complete
```

## 14.3 Wait For

`wait for` suspends the current task until an async operation completes:

```lp
response = wait for get url.
data = wait for fetch_data(url).
results = wait for all(task1, task2, task3).
```

`wait for` MUST only appear inside `async` functions or event handlers.

## 14.4 Async HTTP

```lp
use network.

async function fetch_user(id),
    response = wait for get ("https://api.example.com/users/" with id).
    return json.parse(response.body).
..

user = wait for fetch_user(42).
```

## 14.5 Concurrent Tasks

### 14.5.1 Spawning Tasks

```lp
task = spawn fetch_data(url).
result = wait for task.
```

### 14.5.2 Waiting for Multiple

```lp
@ Wait for all
results = wait for all(
    fetch_data(url1),
    fetch_data(url2),
    fetch_data(url3)
).

@ Wait for first
result = wait for any(task1, task2).
```

### 14.5.3 Task Handles

```lp
task = spawn long_running_work().
task.cancel().
status = task.status().    @ Running, Completed, Cancelled, Failed
```

## 14.6 Async Event Handlers

Event handlers can await without blocking the event loop:

```lp
on button.clicked,
    data = wait for fetch(url).
    update_display(data).
..
```

## 14.7 Synchronization Primitives

### 14.7.1 Mutex

```lp
lock = Mutex().
lock.acquire().
try,
    shared_data += 1.
finally,
    lock.release().
..
```

Scoped lock (v0.2):

```lp
with lock,
    shared_data += 1.
..
```

### 14.7.2 Channel

```lp
channel = Channel<Int>(capacity = 100).

@ Producer
spawn,
    for i in 0..100,
        channel.send(i).
    ..
    channel.close().
..

@ Consumer
while true,
    value = wait for channel.receive().
    if value == null, break.
    process(value).
..
```

### 14.7.3 Atomic Types

```lp
counter = AtomicInt(0).
counter.fetch_add(1).
value = counter.load().
```

## 14.8 Thread Safety

- `Send` trait: types safe to transfer between threads
- `Sync` trait: types safe to share between threads via reference
- The compiler MUST enforce Send/Sync bounds on spawned tasks

## 14.9 Async Runtime

The async runtime is provided by the `async` module:

```lp
use async.

async.run(main()).    @ Run async main and block until complete
```

Configuration:

```lp
async.configure(
    workers = 4.
    stack_size = 1024 * 1024.
).
```

## 14.10 Sleep and Timers

```lp
wait for sleep(seconds = 1.5).

timer = Timer(interval = 1000).
on timer.elapsed,
    print "Tick".
..
timer.start().
```

## 14.11 Parallel Iteration

```lp
results = parallel for item in items,
    wait for process(item).
..
```

Results maintain input order.

## 14.12 Async Streams

```lp
async function data_stream() -> AsyncStream<Int>,
    for i in 0..100,
        yield i.
        wait for sleep(seconds = 0.1).
    ..
..

async for value in data_stream(),
    print value.
..
```

## 14.13 Error Handling in Async Code

Errors in async functions propagate through `wait for`:

```lp
async function fetch_safe(url),
    try,
        return wait for get url.
    catch error: NetworkError,
        print "Network error: " with error.message.
        return null.
    ..
..
```

Uncaught errors in spawned tasks are reported to the async runtime error handler.

## 14.14 Cancellation

Tasks support cooperative cancellation:

```lp
async function long_task(cancel: CancellationToken),
    repeat forever,
        if cancel.is_cancelled(),
            return.
        ..
        wait for do_step().
    ..
..
```

## 14.15 Performance Guidelines

1. Use `spawn` for independent parallel work.
2. Prefer `wait for all` over sequential awaits when operations are independent.
3. Avoid blocking operations in async functions — use async I/O.
4. Keep event handlers fast; offload heavy work to spawned tasks.
