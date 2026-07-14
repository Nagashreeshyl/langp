# Chapter 19 — Runtime & Memory

## 19.1 Overview

The Lang.P runtime provides memory management, execution, foreign function interface, and platform abstraction. The runtime is embedded in every compiled Lang.P binary.

## 19.2 Execution Models

Lang.P supports two execution modes:

| Mode | Description | Use case |
|------|-------------|----------|
| **Interpreted** | Tree-walking interpreter via AST/bytecode | Development, REPL, scripting |
| **Compiled** | Native machine code via LLVM backend | Production, performance-critical |

Both modes share the same runtime and semantics.

## 19.3 Compilation Pipeline

```
Source (.lp)
    │
    ▼
┌─────────┐
│  Lexer  │  Token stream
└────┬────┘
     ▼
┌─────────┐
│  Parser │  Abstract Syntax Tree
└────┬────┘
     ▼
┌──────────────┐
│  Semantic    │  Typed AST + diagnostics
│  Analyzer    │
└────┬─────────┘
     ▼
┌──────────────┐
│  Optimizer   │  Optimized IR
└────┬─────────┘
     ▼
┌──────────────┐     ┌──────────────┐
│  Interpreter │ OR  │  Compiler    │  Native binary
│  (bytecode)  │     │  (LLVM IR)   │
└────┬─────────┘     └──────┬───────┘
     │                      │
     └──────────┬───────────┘
                ▼
         ┌──────────────┐
         │   Runtime    │
         │  (GC, FFI)   │
         └──────────────┘
```

## 19.4 Intermediate Representation (IR)

Lang.P IR is a typed, SSA-based intermediate representation:

```
function @add(a: i64, b: i64) -> i64 {
entry:
    %0 = add i64 %a, %b
    ret i64 %0
}
```

IR properties:

- Static single assignment (SSA) form
- Typed operations matching Lang.P type system
- Platform-independent
- Optimizable (dead code elimination, inlining, constant folding)

## 19.5 Memory Management

### 19.5.1 Garbage Collection

Lang.P uses a **generational, concurrent garbage collector**:

| Generation | Description | Collection frequency |
|------------|-------------|---------------------|
| Young | New allocations | Frequent, stop-the-world (fast) |
| Old | Long-lived objects | Infrequent, concurrent |
| Permanent | Static constants, types | Never collected |

GC properties:

- **Concurrent marking** — minimal pause times.
- **Write barriers** — track old→young references.
- **Finalizers** — `destroy()` methods called before collection.
- **GC tuning** — `--gc-threshold`, `--gc-debug` flags.

### 19.5.2 Stack vs Heap

| Location | Types |
|----------|-------|
| Stack | Primitives (`Int`, `Float64`, `Bool`, `Char`), references |
| Heap | `String`, collections, objects, closures |

Value types (structs without heap allocation) MAY be stack-allocated when the optimizer determines it is safe.

### 19.5.3 Memory Safety

- **No manual memory management** — no `malloc`/`free`.
- **Null safety** — nullable types require explicit `?` annotation.
- **Bounds checking** — array/list index access is bounds-checked (can be optimized away when proven safe).
- **No dangling pointers** — GC prevents use-after-free.

## 19.6 Calling Conventions

### 19.6.1 Lang.P Functions

Arguments passed left-to-right, with the receiver (`self`) as the first argument for methods. Return value in a designated register or via hidden pointer for large returns.

### 19.6.2 Foreign Function Interface (FFI)

Call C functions from Lang.P:

```lp
use ffi.

@ Declare external C function
extern function strlen(s: Pointer<Byte>) -> Int from "libc".

length = strlen(c_string("hello")).
```

Call Lang.P from C:

```c
// Generated header: mylib.h
int64_t langp_add(int64_t a, int64_t b);
void langp_greet(const char* name);
```

FFI rules:

- `extern` functions MUST specify the library name.
- String marshalling: Lang.P `String` ↔ C `const char*`.
- Memory ownership: caller owns arguments; callee owns return values.

## 19.7 Threading Model

- **OS threads** — mapped 1:1 to Lang.P threads.
- **Async tasks** — multiplexed on a thread pool (work-stealing).
- **Thread-local storage** — supported via `thread_local` keyword (v0.2).
- **No global lock** — concurrent GC, lock-free data structures where possible.

## 19.8 Runtime Initialization

```
1. Platform abstraction init (signals, locale)
2. GC init (heap allocation)
3. Standard library init (register built-in modules)
4. Module loader init
5. Execute module initializers (in dependency order)
6. Execute main() or top-level statements
7. Enter event loop (if applicable)
8. Shutdown: finalize, GC sweep, platform cleanup
```

## 19.9 Error Reporting at Runtime

Runtime errors include:

- Stack trace with Lang.P source locations
- Variable values at each frame (in debug mode)
- Suggestion for common errors

```
panic: IndexError: index 5 out of bounds (length 3)
  --> src/main.lp:12:5
   |
12 |     print items[5].
   |           ^^^^^^^^
   |
  stack trace:
    main at src/main.lp:12
    process at src/utils.lp:8
```

## 19.10 Platform Abstraction

The runtime abstracts platform differences:

| Feature | macOS | Windows | Linux |
|---------|-------|---------|-------|
| File paths | POSIX + `/` | Win32 + `\` (normalized) | POSIX |
| Dynamic loading | dlopen | LoadLibrary | dlopen |
| Threads | pthread | Win32 threads | pthread |
| Signals | POSIX signals | SEH | POSIX signals |

Lang.P code is platform-independent unless using platform-specific modules.

## 19.11 Binary Format

Compiled Lang.P binaries:

| Extension | Description |
|-----------|-------------|
| `.lpc` | Lang.P compiled object file |
| (none) | Executable binary |

Object files contain IR metadata, debug symbols, and dependency information for linking.

## 19.12 Debug Information

Debug builds include:

- Source file mapping (IR ↔ source)
- Variable names and types
- Line number tables
- Inline stack frames

Used by the debugger, profiler, and error reporting.

## 19.13 Performance Characteristics

Target performance (compiled mode, relative to C):

| Benchmark | Target |
|-----------|--------|
| Numeric computation | 80-95% of C |
| String processing | 70-85% of C |
| Object-oriented code | 75-90% of C |
| Async I/O | Comparable to Go |
| Startup time | < 50ms (compiled) |

The interpreter runs 10-50x slower than compiled code and is intended for development only.

## 19.14 Resource Limits

Configurable limits:

| Resource | Default | Flag |
|----------|---------|------|
| Stack size | 8 MB | `--stack-size` |
| Heap size | Unlimited (GC) | `--max-heap` |
| Recursion depth | 10,000 | `--max-recursion` |
| Open files | OS limit | — |
| Thread count | 10,000 | `--max-threads` |

Exceeding limits causes a catchable `ResourceError`.
