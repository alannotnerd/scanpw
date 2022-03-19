# `scanpw`

Read a password from standard input

---

## Overview

`scanpw` provides a macro and a function (for more granular error handling) to
facilitate reading passwords from standard input in a secure manner. It expands
to an expression that returns a [`String`][0], so it can be assigned to
a variable or used directly. The macro may take arguments like those to
[`print`][1], which can be used to generate a prompt.

## Examples

### Simple prompt

```rust
let password = scanpw!("Password: ");
```

This results in a prompt that looks like this (where `_` represents where the
user will start typing):

```
Password: _
```

### No prompt

```rust
let password = scanpw!();
```

### Formatted prompt

```rust
let password = scanpw!("Password for {}: ", username);
```

### Custom echo behavior

If the first argument to `scanpw` is an expression of type `Option<char>`
instead of a string literal, it is used to either set a custom replacement
character (like `Some('X')`) or disable echoing entirely (like `None`). For
example:

```rust
// Don't print a '*' for each character the user types
let echo_settings: Option<char> = None;

let password = scanpw!(echo_settings, "Password: ");
```

The default behavior is to echo `*`s for each character entered.

[0]: https://doc.rust-lang.org/std/string/struct.String.html

[1]: https://doc.rust-lang.org/std/macro.print.html
