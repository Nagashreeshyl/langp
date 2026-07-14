# Database

Database connectivity, query execution, and ORM-style access for Lang.P programs.

## Supported Backends

| Backend | Driver | Status |
|---------|--------|--------|
| PostgreSQL | `database.postgres` | Planned |
| SQLite | `database.sqlite` | Planned |
| MySQL | `database.mysql` | Planned |
| MongoDB | `database.mongodb` | Planned |

## Quick Example

```lp
use database.

db = database.connect("postgresql://localhost/mydb").
users = db.query("SELECT * FROM users WHERE age > ?", 18).
db.table("users").insert(name = "Naga", age = 25).
```

## Status

Part of the standard library. See [Chapter 16 — Standard Library](../docs/spec/16-standard-library.md).

## Dependencies

- `runtime/` — connection pooling and async I/O
