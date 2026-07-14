# Chapter 2 — Lexical Structure

## 2.1 Source Files

A Lang.P source file MUST:

- Use the `.lp` file extension.
- Be encoded in UTF-8 without a byte-order mark (BOM).
- Use Unix line endings (`\n`) in canonical form; tools MUST accept `\r\n`.

## 2.2 Comments

Comments begin with `@` and extend to the end of the line.

```lp
@ This is a comment.
print "Hello".  @ Inline comments are allowed after statements.
```

Rules:

- Comments MUST NOT nest.
- Comments are treated as whitespace by the parser.
- Multi-line comments require a `@` on each line:

```lp
@ Line one of the comment.
@ Line two of the comment.
```

There is no block comment syntax. This is intentional — `@` reads as "note" or "annotation" and is visually distinct from code.

## 2.3 Whitespace

Whitespace consists of:

- Space (`U+0020`)
- Tab (`U+0009`) — discouraged; formatters SHOULD convert tabs to spaces
- Newline (`U+000A`, `U+000D U+000A`)

Whitespace is significant **only** for indentation within blocks (see §2.7).

## 2.4 Statement Terminator

Every statement MUST end with a period (`.`).

```lp
age = 18.
print "Hello".
```

The period is the statement terminator, analogous to a semicolon in C-family languages but chosen for readability — it marks the end of an instruction, like a sentence.

### 2.4.1 Trailing Period in Blocks

The closing block marker `..` does NOT require a preceding period on the same line, but the last statement inside a block MUST still end with `.`:

```lp
if true,
    print "Yes".   @ Required period
..
```

## 2.5 Block Delimiters

Blocks begin with a comma (`,`) and end with double-period (`..`).

```lp
if age >= 18,
    print "Adult".
..
```

Rules:

- The comma MUST appear at the end of the line that opens the block (or after the block header expression).
- The `..` MUST appear at the beginning of the indentation level of the block opener (dedented).
- Blocks MUST contain at least one statement or a blank-line placeholder comment.

### 2.5.1 Indentation

- Indentation MUST use spaces only.
- Each indentation level is **4 spaces**.
- The compiler, formatter, and IDE MUST enforce consistent indentation.
- Mixing indentation levels within a block is a **compile error**.

The IDE automatically inserts indentation when `,` is typed and dedents when `..` is typed.

## 2.6 Identifiers

```
identifier ::= (letter | "_") (letter | digit | "_")*
letter     ::= "a"..."z" | "A"..."Z"
digit      ::= "0"..."9"
```

Rules:

- Identifiers are case-sensitive: `name` and `Name` are distinct.
- Identifiers MUST NOT begin with a digit.
- Identifiers MUST NOT equal any reserved keyword (§2.8).
- Unicode identifiers are NOT supported in v0.1; this MAY be added in a future version.
- By convention, `SCREAMING_SNAKE_CASE` is used for module-level constants.

## 2.7 Literals

### 2.7.1 Integer Literals

```
integer_literal ::= decimal_integer | hex_integer | binary_integer | octal_integer
decimal_integer ::= digit+
hex_integer     ::= "0x" hex_digit+
binary_integer  ::= "0b" ("0" | "1")+
octal_integer   ::= "0o" digit+
```

Examples: `42`, `0xFF`, `0b1010`, `0o755`

Integers are arbitrary-precision at compile time; runtime integers are platform-sized (`Int`) or explicitly `Int64`.

### 2.7.2 Float Literals

```
float_literal ::= digit+ "." digit+ (exponent)? | digit+ exponent
exponent        ::= ("e" | "E") ("+" | "-")? digit+
```

Examples: `3.14`, `95.6`, `1.0e-10`, `2E+5`

Default float type is `Float64`.

### 2.7.3 Boolean Literals

```
boolean_literal ::= "true" | "false"
```

### 2.7.4 String Literals

```
string_literal ::= '"' (escape_sequence | non_quote_char)* '"'
                 | "'" (escape_sequence | non_apostrophe_char)* "'"
```

Both double-quoted and single-quoted strings are supported. They are equivalent.

Escape sequences:

| Sequence | Meaning |
|----------|---------|
| `\n` | Newline |
| `\t` | Tab |
| `\r` | Carriage return |
| `\\` | Backslash |
| `\"` | Double quote |
| `\'` | Apostrophe |
| `\u{XXXX}` | Unicode code point (hex) |

Examples:

```lp
name = "Naga".
greeting = 'Hello'.
path = "C:\\Users\\Naga".
emoji = "\u{1F600}".
```

Raw strings (no escape processing) use triple quotes:

```
raw_string ::= '"""' any_char* '"""'
```

```lp
regex = """\d+\.\d+""".
```

### 2.7.5 Character Literals

```
character_literal ::= "'" (escape_sequence | non_apostrophe_char) "'"
```

Character literals represent a single Unicode scalar value of type `Char`.

```lp
letter = 'A'.
newline = '\n'.
```

### 2.7.6 Null Literal

```
null_literal ::= "null"
```

`null` represents the absence of a value for nullable types.

## 2.8 Reserved Keywords

The following tokens are reserved and MUST NOT be used as identifiers:

```
and         as          async       await       break
catch       continue    else        enum        false
finally     for         forever     function    if
in          input       interface   let         match
not         null        on          or          otherwise
repeat      return      self        static      super
this        true        try         type        use
wait        while       with
```

Additionally, these contextual keywords are reserved in their syntactic positions:

```
otherwise if    @ Two-token keyword for else-if
repeat forever  @ Two-token keyword for infinite loop
wait for        @ Two-token keyword for async await
```

### 2.8.1 Input Type Keywords

The following tokens are **contextual keywords** — reserved only immediately after the `input` keyword (see [Chapter 6 §6.15](06-expressions.md#615-input-expression)). Outside that position, they MAY be used as identifiers:

```
boolean     color       date        decimal     file
folder      number      password    text
```

Example disambiguation:

```lp
file = input file "Choose a file".    @ `file` (left) is identifier; `file` (after input) is keyword
text = input text "Enter text : ".   @ `text` keyword selects text input mode
```

## 2.9 Operators and Delimiters

| Token | Name |
|-------|------|
| `.` | Statement terminator / member access |
| `,` | Block opener / separator |
| `..` | Block closer |
| `=` | Assignment / default parameter |
| `==` `!=` `<` `>` `<=` `>=` | Comparison |
| `+` `-` `*` `/` `%` | Arithmetic |
| `**` | Exponentiation |
| `&` `\|` `^` `~` | Bitwise |
| `<<` `>>` | Bit shifts |
| `&&` `\|\|` | Logical |
| `!` | Logical not |
| `?` | Nullable / optional |
| `:` | Type annotation / map entry |
| `(` `)` | Grouping / call |
| `[` `]` | Index / list |
| `{` `}` | Dictionary / set (collection literals only) |
| `->` | Function return type annotation |
| `=>` | Lambda / match arm |
| `@` | Comment |
| `..<` | Range (exclusive end) |
| `...` | Spread / variadic |

Note: `+` exists for arithmetic but MUST NOT be used for string concatenation — use `with` (see Chapter 6).

## 2.10 Tokenization Rules

1. The lexer MUST use maximal munch — the longest valid token is chosen.
2. `..` is a single token (block closer), not two periods.
3. `...` is a single token (spread), distinct from `..`.
4. `..<` is a single token (exclusive range).
5. `otherwise if` is tokenized as a single keyword when `otherwise` is followed by whitespace and `if`.
6. `repeat forever` and `wait for` follow the same two-token keyword rule.
7. A period (`.`) at the end of a statement is the terminator, not member access. The lexer uses context: if `.` is followed by whitespace or newline, it is a terminator; if followed by an identifier, it is member access.

### 2.10.1 Disambiguation Example

```lp
user.name = "Naga".     @ `.name` is member access; final `.` is terminator
print user.name.        @ member access then terminator
```

## 2.11 Line Continuation

Statements MUST NOT span multiple lines unless inside a block, parentheses, brackets, or a string literal. There is no line-continuation character.

```lp
@ Valid — inside parentheses
result = calculate(
    value1,
    value2
).

@ Invalid — statement split across lines
print "Hello"
    with name.
```
