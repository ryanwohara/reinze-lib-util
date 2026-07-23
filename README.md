# reinze-lib-util

General-purpose utility command plugin for the [reinze](https://reinze.com) IRC
bot. It builds to a shared library (`.so`) that the `rust-reinze` host loads
dynamically at runtime, providing a math evaluator, per-user color settings, and
temperature conversions.

Output from a `-command` is private (messaged to the caller).

## Commands

### Utility

- `-calc (expression)` — Evaluate a math expression. Supports `+ - * / % ^`,
  parentheses, decimals, comparisons (`> < =`), the constants `pi` and `e`, and
  the functions abs, sqrt, exp, ln, sin, cos, tan, asin, acos, atan, atan2, sinh,
  cosh, tanh, asinh, acosh, atanh, floor, ceil, round, and signum. Numbers may use
  `k`/`m`/`b` suffixes; results show thousands separators (aliases `-calculate`,
  `-calculator`).
- `-colors get | set <fg>[,<bg>] <fg>[,<bg>] | del` — Manage your personal pair of
  highlight colors used to format the bot's output for you (alias `-color`). Each
  color is a two-digit code, optionally followed by a comma and a two-digit
  background code (e.g. `14` or `14,01`). `get` previews your current colors;
  `del` resets to default. Example: `-colors set 14,01 04,01`.
- `-c (temperature)` — Convert Celsius to Fahrenheit, F = C × 1.8 + 32 (alias
  `-c-f`).
- `-f (temperature)` — Convert Fahrenheit to Celsius, C = (F − 32) / 1.8 (alias
  `-f-c`).

## Building

```sh
cargo build --release
```

This produces `target/release/libreinze_lib_util.so`. Install it into the
`rust-reinze` host's `plugins/` directory. Install **atomically** — build to a
temp file on the same filesystem, then `mv`/rename it into place — so the host
never loads a partially written library.
