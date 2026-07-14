# Lang.P Formal Grammar — Complete Reference

**Version:** 0.1.0  
**Phase:** 2 Complete

This document combines the lexical and syntactic grammars with notation rules for human reading. Machine-readable sources:

- [`02-lexical-grammar.ebnf`](02-lexical-grammar.ebnf)
- [`03-syntactic-grammar.ebnf`](03-syntactic-grammar.ebnf)

---

## 1. Notation

See [01-notation.md](01-notation.md) for full details.

- `=` definition, `|` alternative, `[ ]` optional, `{ }` repetition
- `"terminal"` literal token, `identifier` non-terminal
- Synthetic tokens: `INDENT`, `DEDENT`, `STMT_END`, `DOT`, `BLOCK_CLOSE`, `NEWLINE`, `EOF`

---

## 2. Lexical Grammar Summary

### Identifiers and Keywords

```
identifier  = (letter | "_") { letter | digit | "_" }

keyword     = and | as | async | ... | with | input | ...
```

**Compound keywords** (single tokens):

| Written | Token |
|---------|-------|
| `otherwise if` | `KW_OTHERWISE_IF` |
| `repeat forever` | `KW_REPEAT_FOREVER` |
| `wait for` | `KW_WAIT_FOR` |

**Contextual keywords** (after `input` only):

`text` `number` `decimal` `boolean` `password` `file` `folder` `date` `color`

### Literals

| Rule | Examples |
|------|----------|
| `integer_literal` | `42`, `0xFF`, `0b1010`, `0o755` |
| `float_literal` | `3.14`, `1.0e-10` |
| `boolean_literal` | `true`, `false` |
| `null_literal` | `null` |
| `string_literal` | `"hello"`, `'world'` |
| `raw_string` | `"""multi\nline"""` |
| `character_literal` | `'A'`, `'\n'` |

### Operators (maximal munch)

```
== != <= >= << >> ** // && || .. ..< ... -> => ?? += -= ...
+ - * / % & | ^ ~ < > ! ?
```

### Period Disambiguation

```
user.name = "Naga".    →  IDENT DOT IDENT = STRING STMT_END
print user.name.       →  PRINT IDENT DOT IDENT STMT_END
```

---

## 3. Syntactic Grammar Summary

### Program Structure

```ebnf
program     = module, EOF .
module      = { module_item } .
module_item = use_decl | function_decl | type_decl | enum_decl
            | interface_decl | extension_decl | event_handler | statement .
```

### Imports

```ebnf
use_decl = "use", dotted_name, STMT_END .
```

### Functions

```ebnf
function_decl = [ "async" ], [ "inline" ], "function", identifier,
                [ generic_params ], function_sig, COMMA, block, BLOCK_CLOSE .

function_sig  = "(", [ param_list ], ")", [ "->", type_expr ] .
```

Example parse tree input:

```lp
function greet(name),
    print "Hello " with name.
..
```

### Types

```ebnf
type_decl = [ visibility ], "type", identifier, [ generic_params ],
            [ type_inheritance ], type_body .

type_body = COMMA, NEWLINE, INDENT, { type_member }, DEDENT, BLOCK_CLOSE .
```

### Statements

| Statement | Syntax |
|-----------|--------|
| Assignment | `target = expr.` |
| Print | `print [inline] expr { ("," \| "with") expr }.` |
| Return | `return [ expr_list ].` |
| If | `if expr, block { elif } [ else ] ..` |
| Repeat | `repeat expr times [as id], block ..` |
| Forever | `repeat forever, block ..` |
| For | `for binding in expr, block ..` |
| While | `while expr, block ..` |
| Try | `try, block { catch } [ finally ] ..` |
| Event | `on expr [, where expr], block ..` |
| Write | `write expr to expr.` |
| Read | `read "path"` (expression) |

### Control Flow Example

```lp
if marks >= 90,
    print "Grade A".
otherwise if marks >= 80,
    print "Grade B".
otherwise,
    print "Grade C".
..
```

Token pattern:

```
IF expr COMMA INDENT ... DEDENT
NEWLINE OTHERWISE_IF expr COMMA INDENT ... DEDENT
NEWLINE OTHERWISE COMMA INDENT ... DEDENT
BLOCK_CLOSE
```

### Input Expression

```ebnf
input_expr = "input", [ input_type_keyword ], string_literal .
```

```lp
name = input "Enter your name : ".
age = input number "Enter your age : ".
```

### With Expression (lowest binary precedence)

```ebnf
with_expr = or_expr, { "with", or_expr } .
```

```lp
message = "Hello " with name with "!".
```

### HTTP and Async

```ebnf
http_expr = "get", expr
          | "post", expr, "with", expr
          | "wait for", expr .
```

### Object Creation

```lp
@ Positional
user = User("Naga", 25).

@ Named-field block
browser = Browser(),
    name = "Nova".
    homepage = "https://google.com".
..
```

```ebnf
object_creation   = type_expr, "(", [ arg_list ], ")" .
named_object_body = type_expr, "(", ")", COMMA, block, BLOCK_CLOSE .
```

---

## 4. Expression Precedence (high → low)

| Level | Operators / forms |
|-------|-------------------|
| 1 | Postfix: `()` `[]` `.` |
| 2 | Unary: `+` `-` `~` `not` `!` |
| 3 | `**` |
| 4 | `*` `/` `%` `//` |
| 5 | `+` `-` |
| 6 | `<<` `>>` |
| 7 | `&` |
| 8 | `^` |
| 9 | `\|` |
| 10 | Comparisons, `is` / `is not` |
| 11 | `and` / `&&` |
| 12 | `or` / `\|\|` |
| 13 | `with` |
| 14 | `??` (postfix) |

---

## 5. Block Indentation Algorithm

1. Header line ends with `,` → emit `COMMA`, expect indented body
2. Next line indented +4 spaces → emit `INDENT`
3. Body statements at block indent
4. Line with `..` at header indent → emit `DEDENT`, `BLOCK_CLOSE`
5. Inconsistent indent → error `E0101`

---

## 6. Conformance

Valid programs: `tests/conformance/parse/valid/`  
Invalid programs: `tests/conformance/parse/invalid/`

---

## 7. Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-07-14 | Initial grammar from Lang.P spec v0.1.0 |
