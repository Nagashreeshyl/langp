# Glossary

Definitions of terms used throughout the Lang.P specification. Normative keywords appear in lowercase; type names in PascalCase.

## A

**Assignment statement** — A statement of the form `variable = expression.` that binds or rebinds a variable. See [Chapter 7 §7.2](07-statements.md#72-assignment-statement).

## B

**Built-in expression** — A language construct that produces a value without a function call or import, such as `input`, collection literals, or the `with` operator. See [Chapter 6](06-expressions.md).

## C

**Color** — An RGBA color value type returned by `input color` and defined in the `graphics` module. See [Chapter 4 §4.14.3](04-types.md#4143-the-color-type).

**Contextual keyword** — A token that is treated as a keyword only in specific syntactic positions (e.g., `text` after `input`, `otherwise if`). See [Chapter 2 §2.8.1](02-lexical-structure.md#281-input-type-keywords).

**Conformance example** — A code example marked as required behavior that implementations MUST satisfy. See [README](README.md#conformance).

## D

**Date** — A calendar date type (year, month, day) returned by `input date` and defined in the `datetime` module. See [Chapter 4 §4.14.2](04-types.md#4142-the-date-type).

## I

**InputCancelledError** — A `RuntimeError` thrown when the user cancels a native picker (`file`, `folder`, `date`, `color`). See [Chapter 6 §6.15.6](06-expressions.md#6156-error-handling).

**Input expression** — The built-in `input` keyword expression for reading user input from the terminal or native system pickers. No parentheses are used. See [Chapter 6 §6.15](06-expressions.md#615-input-expression).

**Input type keyword** — A contextual keyword after `input` that selects the input mode and return type: `text`, `number`, `decimal`, `boolean`, `password`, `file`, `folder`, `date`, or `color`. See [Chapter 2 §2.8.1](02-lexical-structure.md#281-input-type-keywords).

**InputError** — A `RuntimeError` thrown when input fails after retries, on EOF, or when a picker is unavailable. See [Chapter 6 §6.15.6](06-expressions.md#6156-error-handling).

## P

**Picker input** — Input modes that open a native system dialog: `file`, `folder`, `date`, and `color`. See [Chapter 6 §6.15.3](06-expressions.md#6153-semantics).

**Prompt** — The string literal displayed to the user before input is collected, e.g., `"Enter your name : "`.

## Q

**Quick-fix** — An IDE code action that automatically applies a suggested fix, such as converting generic `input` to typed `input number`. See [Chapter 21 §21.3.2](21-tooling.md#2132-input-type-quick-fix).

## S

**Statement terminator** — The period (`.`) that ends every statement. See [Chapter 2 §2.4](02-lexical-structure.md#24-statement-terminator).

## T

**Type inference** — Compile-time deduction of types without explicit annotations. For `input`, the compiler infers the return type from assignment context and usage. See [Chapter 4 §4.14.5](04-types.md#4145-type-inference-for-default-input).

## W

**W0101** — Compiler warning: input type could be more specific. Emitted when generic `input` is used but usage suggests a typed variant. See [Chapter 6 §6.15.4](06-expressions.md#6154-type-inference).

**Warning** — A compile-time diagnostic that does not prevent compilation. Input-related warnings use the `W01xx` range.
