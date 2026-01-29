# JSON Parser

This is a small, educational JSON parser written in Rust.

- Purpose: my very first JSON parser implementation and my first finished Rust program, built as a study project to learn parsing and Rust.
- Status: proof-of-concept / learning project.

Features

- Supports parsing JSON objects and arrays.
- Recognizes strings, numbers (decimal and negative), booleans (`true` / `false`) and `null`.
- Returns data as defined by the `JsonType` enum (includes dictionaries, arrays, int/floats, bool and nulls).

Limitations

- No support for escaped characters inside strings (e.g. `\"`, `\\`, `\n`).
- Scientific notation for numbers (exponents `e`, `E`) is not implemented and will panic if encountered.
- Not meant for production use, it is a learning exercise and intentionally minimal.
