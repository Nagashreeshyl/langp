# Chapter 15 — I/O & Network

> **Implemented today (v0.1):** File read/write/copy/move/delete work in the interpreter. HTTP, WebSocket, and server APIs in this chapter are **specification only**. See [16 — File System](../manual/16-filesystem.md) and [17 — Networking](../manual/17-networking.md) in the manual.

## 15.1 Filesystem

Lang.P provides natural-language-style file I/O.

### 15.1.1 Reading Files

```lp
use filesystem.

text = read "settings.txt".
bytes = read_bytes "image.png".
lines = read_lines "data.csv".
```

With error handling:

```lp
try,
    text = read "settings.txt".
catch error: FileNotFoundError,
    text = "".
..
```

### 15.1.2 Writing Files

```lp
write text to "backup.txt".
write_bytes data to "output.bin".
append line to "log.txt".
```

### 15.1.3 File Operations

```lp
exists = file_exists("config.json").
size = file_size("data.bin").
delete "temp.txt".
copy "source.txt" to "dest.txt".
move "old.txt" to "new.txt".
rename "old.txt" to "new.txt".
```

### 15.1.4 Directory Operations

```lp
create_dir "logs".
create_dir_all "path/to/nested/dir".
delete_dir "temp".
list_files "src/".
list_dirs ".".
```

### 15.1.5 Path Operations

```lp
use filesystem.path.

full = path.join("src", "main.lp").
parent = path.parent("/src/main.lp").
name = path.filename("/src/main.lp").
ext = path.extension("main.lp").
absolute = path.resolve("../config.json").
```

### 15.1.6 File Watching

```lp
watcher = watch "config.json".
on watcher.changed,
    reload_config().
..
```

## 15.2 Standard I/O

### 15.2.1 Built-in Input Expression

Interactive input is provided by the built-in `input` expression (see [Chapter 6 §6.15](06-expressions.md#615-input-expression)). This is the **preferred** way to read user input:

```lp
name = input "Enter your name : ".
age = input number "Enter your age : ".
password = input password "Enter your password : ".
resume = input file "Choose your resume".
```

Do **not** use function-call syntax — Lang.P has no `input()` function:

```lp
@ Invalid
name = input("Enter name: ").

@ Correct
name = input "Enter name: ".
```

### 15.2.2 Low-Level Stream I/O

For library authors, scripts, and non-interactive pipelines, lower-level stream functions remain available:

```lp
@ Read a line without the input expression (stdlib / IO module)
name = read_line("Enter name: ").
password = read_line_masked("Password: ").

@ Output
print "Hello".
print inline "Loading".
write stderr "Error occurred".
```

These functions are equivalent to `input text` and `input password` respectively but lack type validation, inference warnings, and IDE quick-fix support. Beginner-facing documentation and tutorials MUST use `input` instead.

## 15.3 HTTP Client

Natural-language HTTP syntax:

```lp
use network.

@ GET
response = get "https://google.com".
response = get url.
response = get (url with "?q=" with query).

@ POST
response = post "https://api.example.com" with data.
response = post url with json_body.

@ Other methods
response = put url with data.
response = delete url.
response = patch url with data.
```

### 15.3.1 Request Options

```lp
response = get url,
    headers = {"Authorization": "Bearer " with token}.
    timeout = 30.
    follow_redirects = true.
..
```

### 15.3.2 Response Object

```lp
response.status.        @ 200
response.body.          @ String
response.headers.       @ Dictionary<String, String>
response.json().        @ Parsed JSON
response.ok.            @ true if 200-299
```

### 15.3.3 Async HTTP

```lp
response = wait for get url.
response = wait for post url with data.
```

## 15.4 HTTP Server

```lp
use network.

function handle(request),
    return response(200, body = "Hello").
..

server = Server(port = 8080).

on server.request,
    reply = handle(server.request).
    server.respond(reply).
..

server.start().
print "Listening on port 8080".
```

### 15.4.1 Route Handling

```lp
server = Server(port = 8080).

on server.request where server.request.path == "/",
    server.respond(response(200, body = "Home")).
..

on server.request where server.request.path == "/api/users",
    users = get_users().
    server.respond(json_response(users)).
..
```

## 15.5 WebSocket

```lp
use network.

socket = WebSocket("wss://echo.example.com").

on socket.message,
    print "Received: " with socket.message.data.
..

on socket.connected,
    socket.send("Hello").
..

socket.connect().
```

## 15.6 TCP/UDP

```lp
@ TCP
listener = TcpListener.bind("0.0.0.0:8080").
on listener.connection,
    handle_connection(listener.connection).
..

@ UDP
socket = UdpSocket.bind("0.0.0.0:9000").
data, addr = socket.receive_from().
socket.send_to(response, addr).
```

## 15.7 DNS

```lp
use network.dns.

addresses = resolve("example.com").
```

## 15.8 JSON

```lp
use json.

data = json.parse('{"name": "Naga", "age": 25}').
text = json.stringify(data).
pretty = json.stringify(data, indent = 2).

@ Typed parsing
user = json.parse_as('{"name": "Naga"}', User).
```

## 15.9 Serialization

Generic serialization beyond JSON:

```lp
use serialization.

bytes = serialize(value).
value = deserialize(bytes, Type).
```

## 15.10 Streaming I/O

```lp
stream = open_stream("large_file.dat").
while true,
    chunk = stream.read(4096).
    if chunk.is_empty(), break.
    process(chunk).
..
stream.close().
```

Async streaming:

```lp
async for chunk in async_read_stream("large_file.dat"),
    wait for process(chunk).
..
```

## 15.11 Compression

```lp
use compression.

compressed = gzip.compress(data).
original = gzip.decompress(compressed).
```

## 15.12 SSL/TLS

TLS is enabled by default for HTTPS. Custom certificates:

```lp
response = get url,
    tls = TlsConfig(
        verify = true.
        ca_cert = read "ca.pem".
    ).
..
```

## 15.13 URL Handling

```lp
use network.url.

parsed = url.parse("https://example.com/path?q=1").
print parsed.host.
print parsed.path.
print parsed.query["q"].

built = url.build(
    scheme = "https".
    host = "example.com".
    path = "/api".
    query = {"page": "1"}.
).
```
