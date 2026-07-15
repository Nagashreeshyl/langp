# Lang.P Architecture v0.2

**Status:** Implementation guide  
**Scope:** OOP, modules, filesystem stdlib, package manager  
**Constraint:** No changes to frozen surface syntax (`.`, `,`, `..`, `@`)

---

## 1. System overview

```
┌──────────────────────────────────────────────────────────────────────────┐
│                           Developer surface                              │
│  lang run | lang check | lang init | lang install | lang build | …       │
└───────────────────────────────────┬──────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼──────────────────────────────────────┐
│  langc (CLI library)          │  langpm (package manager crate)          │
│  parse → check → run          │  manifest · lock · resolver · cache      │
└───────────────────────────────────┬──────────────────────────────────────┘
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        │                           │                           │
┌───────▼───────┐         ┌─────────▼─────────┐       ┌────────▼────────┐
│ langp-semantic│         │ langp-interpreter │       │ ModuleLoader    │
│ types · use   │         │ oop · eval · stdlib│       │ .lp · stdlib   │
└───────────────┘         └─────────┬─────────┘       └─────────────────┘
                                    │
                          ┌─────────▼─────────┐
                          │  langp-runtime    │
                          │  Value · errors   │
                          └───────────────────┘
```

---

## 2. Object-oriented programming

### 2.1 Runtime model

| Concept | Runtime representation |
|---------|------------------------|
| Type definition | `TypeDef` in `TypeRegistry` (interpreter) |
| Type reference | `Value::LangType(Rc<TypeDef>)` — callable as `User()` |
| Instance | `Value::Instance(Rc<InstanceData>)` — fields + type link |
| Method | `UserFunction` bound with `self` in environment |
| Static member | Stored on `TypeDef.static_fields` |
| Inheritance | `TypeDef.parent: Option<Rc<TypeDef>>` — field/method lookup chain |

### 2.2 Initialization pipeline

```
Program load
    → register TypeDecl items in TypeRegistry
    → resolve extends (parent link)
    → env.define("User", Value::LangType(...))

User() expression
    → allocate Instance with default field values
    → apply positional/named constructor args
    → call init() if defined
    → return Instance
```

### 2.3 Method dispatch

```
user.greet()
    → eval user → Instance
    → lookup greet on TypeDef (walk parent chain)
    → create child env with self = Instance
    → eval function body
```

### 2.4 Feature matrix

| Feature | v0.2 | Mechanism |
|---------|------|-----------|
| Fields | ✓ | `TypeMember::Field` → instance HashMap |
| Methods | ✓ | `TypeMember::Function` → bound call |
| Constructors | ✓ | `init` method + `User()` syntax |
| Inheritance | ✓ | `extends` in type header (EBNF) |
| Interfaces | Partial | Structural check at semantic layer |
| Visibility | Partial | Parse `public`/`private`/`internal` prefix |
| Static members | ✓ | `static` prefix on fields/methods |
| Generics | Partial | Semantic validation only |
| Properties | Planned | Getter/setter blocks in spec |

---

## 3. Module system

### 3.1 Import resolution

```
use filesystem.
    → ModuleLoader.load("filesystem")
    → check cache
    → if stdlib: built-in Rust module
    → if project: resolve src/<path>.lp or packages/<name>/lib.lp
    → detect circular imports via loading stack
    → env.define("filesystem", Value::Module(...))
```

### 3.2 Module sources (priority order)

1. **Standard library** — compiled into `langp-interpreter` (`stdlib/`)
2. **Project modules** — `src/**/*.lp` relative to project root
3. **Installed packages** — `~/.cache/langp/packages/<name>-<version>/`
4. **Lock file paths** — pinned in `langp.lock`

### 3.3 Namespace model

`Value::Module` holds `HashMap<String, Value>` exports.

Access: `filesystem.exists("path")` — member call on module.

Top-level `read` / `write` statements remain language builtins; `use filesystem.` registers extended API (exists, list, mkdir, …).

### 3.4 Circular dependency detection

```rust
loading: HashSet<String>  // module paths currently being loaded
if loading.contains(path) → CircularImportError
```

---

## 4. Filesystem standard library

Built-in module `filesystem` (also enables host I/O when imported).

| API | Implementation |
|-----|----------------|
| `read(path)` | `std::fs::read_to_string` |
| `read_bytes(path)` | `std::fs::read` |
| `write(text, path)` | `std::fs::write` |
| `append(text, path)` | OpenOptions append |
| `copy(src, dst)` | `std::fs::copy` |
| `move(src, dst)` | `std::fs::rename` |
| `delete(path)` | `remove_file` |
| `exists(path)` | `Path::exists` |
| `list(dir)` | `read_dir` → List of names |
| `create_folder(path)` | `create_dir_all` |
| `remove_folder(path)` | `remove_dir_all` |

Language-level statements (`read "x".`, `write v to "y".`) delegate to the same `fs` helpers.

---

## 5. Package manager (`langpm`)

### 5.1 Manifest (`langp.toml`)

```toml
[package]
name = "my-app"
version = "0.1.0"
entry = "main.lp"

[dependencies]
filesystem = "0.1"
navigator = { git = "https://github.com/Nagashreeshyl/langp-navigator", branch = "main" }
my-lib = { path = "../my-lib" }
```

Also accepts legacy `lang.toml` filename.

### 5.2 Lock file (`langp.lock`)

Pins exact resolved versions:

```toml
[[package]]
name = "filesystem"
version = "0.1.0"
source = "registry"

[[package]]
name = "navigator"
version = "1.0.0"
source = "git"
checksum = "..."
```

### 5.3 Dependency resolution

1. Read manifest dependencies
2. Fetch registry index (`~/.cache/langp/registry/index.toml`) or clone git
3. Resolve semver constraints (`semver` crate)
4. Detect conflicts (two incompatible versions of same name)
5. Write lock file
6. Install to cache

### 5.4 CLI command routing

| Command | Handler |
|---------|---------|
| `lang init [name]` | Create manifest + main.lp |
| `lang install [pkg]` | Resolve + download deps |
| `lang remove <pkg>` | Remove from manifest + lock |
| `lang update [pkg]` | Bump deps within constraints |
| `lang search <query>` | Search local registry index |
| `lang publish` | Stub — upload to registry (future) |
| `lang login` | Stub — store auth token |
| `lang build` | Build project entry via langc |
| `lang test` | Run `tests/**/*.lp` |
| `lang fmt` | Format check (delegates to lang check) |
| `lang doctor` | Verify toolchain, cache, manifest |
| `lang run <file>` | Existing single-file runner |

### 5.5 Offline cache layout

```
~/.cache/langp/
├── packages/
│   └── navigator-1.0.0/
│       ├── langp.toml
│       └── lib.lp
├── registry/
│   └── index.toml
└── src/                    # git clone cache (install.sh)
```

---

## 6. Semantic analysis extensions

| Check | Description |
|-------|-------------|
| `use` placement | Warn if not at file top |
| Duplicate imports | Warning |
| Type field types | Validate annotations vs defaults |
| `extends` cycle | Error on circular inheritance |
| Interface satisfaction | Method signature match (structural) |
| Module exports | Undefined export references |

---

## 7. Testing strategy

| Suite | Location | Covers |
|-------|----------|--------|
| OOP integration | `interpreter/tests/oop.rs` | types, init, methods, extends |
| Modules | `interpreter/tests/modules.rs` | use, stdlib, circular detect |
| Filesystem | `interpreter/tests/filesystem.rs` | read/write/exists/list |
| Package manager | `langpm/tests/` | manifest, resolver, lock |
| Conformance | `tests/conformance/parse/valid/` | parse fixtures |

---

## 8. Implementation phases

| Phase | Deliverable | Status |
|-------|-------------|--------|
| **A** | Architecture doc (this file) | ✓ |
| **B** | Runtime `Instance`, `Module`, `LangType` | In progress |
| **C** | OOP: register types, construct, methods, self | In progress |
| **D** | Module loader + filesystem stdlib | In progress |
| **E** | langpm manifest/lock/install/init/doctor | In progress |
| **F** | Inheritance, static, visibility | In progress |
| **G** | Interfaces, generics (semantic) | Partial |
| **H** | Registry publish/search (network) | Future |

---

## 9. Related documents

- [Grammar Freeze v1](spec/GRAMMAR-FREEZE-v1.md)
- [Object Model (Ch. 10)](spec/10-object-model.md)
- [Modules (Ch. 11)](spec/11-modules-imports.md)
- [Package System (Ch. 20)](spec/20-package-system.md)
- [Tech Stack](TECH-STACK.md)
