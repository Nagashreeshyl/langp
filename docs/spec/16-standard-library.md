# Chapter 16 — Standard Library

> **Implemented today (v0.1):** See [Language Reference — implemented subset](../../guides/LANGUAGE-REFERENCE.md) for commands that actually work in the current interpreter. This chapter describes the full planned standard library.

## 16.1 Overview

The Lang.P standard library provides production-quality modules for common tasks. All stdlib modules follow the language philosophy: APIs read like natural instructions.

## 16.2 Core (`core`)

Built-in types and functions available without import:

| Function | Description |
|----------|-------------|
| `print(value...)` | Print to stdout |
| `assert(condition, message?)` | Debug assertion |
| `panic(message)` | Unrecoverable error |
| `to_string(value)` | Convert to string |
| `parse_int(text)` | Parse integer |
| `parse_float(text)` | Parse float |
| `len(collection)` | Length of collection |
| `type_of(value)` | Runtime type name |
| `exit(code?)` | Exit program |

Built-in types: `Int`, `Float64`, `Bool`, `Char`, `String`, `List`, `Dictionary`, `Set`, `Tuple`, `Object`, `Null`.

Built-in expressions (no import required):

| Expression | Description |
|------------|-------------|
| `input "prompt"` | Read user input (type inferred or `String`) |
| `input text "prompt"` | Read text line from stdin |
| `input number "prompt"` | Read validated integer |
| `input decimal "prompt"` | Read validated decimal |
| `input boolean "prompt"` | Read yes/no confirmation |
| `input password "prompt"` | Read masked text |
| `input file "prompt"` | Native file picker |
| `input folder "prompt"` | Native folder picker |
| `input date "prompt"` | Native date picker |
| `input color "prompt"` | Native color picker |

See [Chapter 6 §6.15](06-expressions.md#615-input-expression) and [Chapter 4 §4.14](04-types.md#414-input-expression-types).

## 16.3 Collections (`collections`)

```lp
use collections.

@ List operations
list = list.of(1, 2, 3).
list.map((x) => x * 2).
list.filter((x) => x > 0).
list.reduce(0, (acc, x) => acc + x).
list.sort().
list.reverse().
list.unique().

@ Dictionary operations
dict = dict.of("a", 1, "b", 2).
dict.keys().
dict.values().
dict.entries().
dict.merge(other).

@ Set operations
set = set.of(1, 2, 3).
set.union(other).
set.intersection(other).
set.difference(other).
```

## 16.4 Math (`math`)

```lp
use math.

math.abs(-5).
math.sqrt(16).
math.pow(2, 10).
math.floor(3.7).
math.ceil(3.2).
math.round(3.5).
math.min(a, b).
math.max(a, b).
math.sin(angle).
math.cos(angle).
math.log(value).
math.random().
math.random_int(1, 100).
```

Constants: `math.PI`, `math.E`.

## 16.5 DateTime (`datetime`)

```lp
use datetime.

now = datetime.now().
today = datetime.today().
parsed = datetime.parse("2026-07-14", format = "%Y-%m-%d").
formatted = now.format("%Y-%m-%d %H:%M:%S").
duration = datetime.Duration(hours = 2, minutes = 30).
future = now + duration.
```

## 16.6 Crypto (`crypto`)

```lp
use crypto.

hash = crypto.sha256(data).
hash = crypto.md5(text).
hmac = crypto.hmac_sha256(key, data).
random_bytes = crypto.random_bytes(32).
uuid = crypto.uuid4().
```

## 16.7 Terminal (`terminal`)

The `terminal` module provides advanced terminal formatting and display — colored output, tables, progress bars, and screen control. It does **not** replace the built-in `input` expression for reading user input.

```lp
use terminal.

@ For basic input, use the built-in input expression instead:
@ name = input "Enter your name : ".

@ Terminal module — formatting and display
terminal.print_colored("Error", color = red).
terminal.clear().
terminal.set_title("My App").
terminal.progress_bar(current, total).
table = terminal.Table().
table.add_row("Name", "Age").
table.add_row("Naga", "25").
table.render().
```

For masked password input at the stdlib level, prefer `input password "..."` over `read_line_masked`. The `terminal` module MAY offer `terminal.prompt()` for styled prompts in v0.2, but `input` remains the canonical beginner API.

## 16.8 Testing (`testing`)

```lp
use testing.

test "addition works",
    assert add(2, 3) == 5.
.

test "division by zero throws",
    assert_throws(DivisionError, function(),
        divide(1, 0).
    ).
.

@ Run with: lang test
testing.run().
```

Test functions:

| Function | Description |
|----------|-------------|
| `test(name, body)` | Define a test case |
| `assert_eq(a, b)` | Assert equality |
| `assert_ne(a, b)` | Assert inequality |
| `assert_throws(type, body)` | Assert exception |
| `assert_true(condition)` | Assert true |
| `assert_false(condition)` | Assert false |

## 16.9 Database (`database`)

```lp
use database.

db = database.connect("postgresql://localhost/mydb").

users = db.query("SELECT * FROM users WHERE age > ?", 18).
db.execute("INSERT INTO users (name) VALUES (?)", "Naga").

@ ORM-style
users = db.table("users").where("age", ">", 18).all().
db.table("users").insert(name = "Naga", age = 25).
```

Supported backends (via drivers):

- PostgreSQL
- SQLite
- MySQL
- MongoDB (document)

## 16.10 Graphics (`graphics`)

2D graphics primitives (v0.2):

```lp
use graphics.

canvas = Canvas(width = 800, height = 600).
canvas.draw_rect(x = 10, y = 10, width = 100, height = 50, color = blue).
canvas.draw_circle(x = 200, y = 200, radius = 50, color = red).
canvas.draw_text("Hello", x = 300, y = 300, font = "Arial", size = 24).
canvas.save("output.png").
```

## 16.11 Audio (`audio`)

```lp
use audio.

sound = audio.load("notification.wav").
sound.play().
audio.record(duration = 5.0, output = "recording.wav").
```

## 16.12 Video (`video`)

```lp
use video.

clip = video.load("intro.mp4").
clip.play().
video.encode(frames, output = "output.mp4", fps = 30).
```

## 16.13 Environment (`env`)

```lp
use env.

key = env.get("API_KEY").
key = env.get("PORT", default = "8080").
home = env.home().
cwd = env.cwd().
env.set("DEBUG", "true").    @ Set env var for child processes
```

## 16.14 Reflection (`reflect`)

```lp
use reflect.

type_name = reflect.type_of(value).
fields = reflect.fields(value).
reflect.call_method(value, "greet", ["Naga"]).
```

## 16.15 Regular Expressions

```lp
use regex.

pattern = regex.compile("\\d+").
matches = pattern.findall("abc123def456").
matched = pattern.match("123").
replaced = pattern.replace("a1b2c3", "X").
```

## 16.16 Logging

```lp
use logging.

logger = logging.get("myapp").
logger.info("Application started").
logger.warning("Low memory").
logger.error("Connection failed").
logger.debug("Processing item " with id).
```

Configuration:

```lp
logging.configure(
    level = logging.INFO.
    format = "{time} [{level}] {message}".
    output = "app.log".
).
```

## 16.17 Standard Library Design Principles

1. **Read like instructions** — `read "file.txt"`, `write data to "output.txt"`.
2. **Sensible defaults** — minimal configuration for common cases.
3. **Progressive disclosure** — simple API first, advanced options available.
4. **Consistent naming** — snake_case functions, PascalCase types.
5. **Documented examples** — every public function includes an example.
6. **Tested** — 100% unit test coverage for stdlib.

## 16.18 Stdlib Versioning

The standard library version matches the language version. Stdlib modules follow semver within the language release cycle. Breaking changes to stdlib require a major language version bump.
