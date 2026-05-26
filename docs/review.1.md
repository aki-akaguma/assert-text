# Code Review Report: assert-text

Thank you for requesting this code review. The `assert-text` crate is a well-designed, lightweight assertion utility that provides beautiful, colored, GitHub-style diffs when assertions fail. The code is highly compact and functional, and clippy passes with zero warnings.

During our in-depth code review, we identified several critical bugs (specifically regarding UTF-8 safety), performance optimizations, and API design improvements that will make the library more robust, efficient, and compatible with the broader Rust ecosystem.

---

## 1. Critical UTF-8 Slicing Panics (Correctness & Safety)

### Description
In Rust, slicing a `&str` using byte indices that do not lie on character boundaries will cause an immediate panic. The macros `assert_text_starts_with!` and `assert_text_ends_with!` slice the `$left` string using byte indices determined by the byte length of `$right`, which is highly likely to panic when dealing with multi-byte UTF-8 characters (e.g., Emojis, CJK characters, accented letters).

#### In `assert_text_starts_with!`:
```rust
let ll = $left.len();
let rl = $right.len();
let orig = $right;
let edit = &$left[0..ll.min(rl)]; // PANIC if ll.min(rl) is not a char boundary
```
If `$left` starts with a multi-byte character (like `"あ"`, 3 bytes) and `$right` is a shorter byte string (like `"a"`, 1 byte), `ll.min(rl)` will be `1`. Slicing `&$left[0..1]` will panic with: `byte index 1 is not a char boundary; it is inside 'あ' (bytes 0..3)`.

#### In `assert_text_ends_with!`:
```rust
let ll = $left.len();
let rl = $right.len();
let orig = $right;
let edit = &$left[if ll > rl { ll - rl } else { 0 }..]; // PANIC if ll - rl is not a char boundary
```
If `$left` ends with multi-byte characters and we subtract the byte length of `$right`, the resulting index `ll - rl` is highly likely to split a UTF-8 character, leading to a panic.

### Proposed Solution
Instead of raw byte slicing, we should determine the safe UTF-8 character boundary based on character count rather than byte count.

For `assert_text_starts_with!`:
```rust
let right_chars = right_val.chars().count();
let limit = left_val.char_indices().nth(right_chars).map(|(idx, _)| idx).unwrap_or(left_val.len());
let edit = &left_val[..limit];
```

For `assert_text_ends_with!`:
```rust
let right_chars = right_val.chars().count();
let total_chars = left_val.chars().count();
let skip_chars = total_chars.saturating_sub(right_chars);
let limit = left_val.char_indices().nth(skip_chars).map(|(idx, _)| idx).unwrap_or(0);
let edit = &left_val[limit..];
```
This approach is highly performant (zero allocations, single-pass iteration) and 100% safe from UTF-8 slicing panics.

---

## 2. Multiple Evaluation of Macro Arguments (Hygiene & Safety)

### Description
All assertions in the crate evaluate `$left` and `$right` multiple times. For example, in `assert_text_starts_with!`:
1. `!$left.starts_with($right)`
2. `$left.len()`
3. `$right.len()`
4. `let orig = $right;`
5. `&$left[0..ll.min(rl)]`

If `$left` or `$right` are complex expressions (e.g., function calls, method chains with side effects, database/file readers), evaluating them multiple times is:
- **Incorrect**: The side effects will be triggered multiple times, and successive evaluations could yield different values (causing false positives or false negatives).
- **Inefficient**: The same expression is executed repeatedly.

### Proposed Solution
Bind the macro arguments exactly once using the standard Rust pattern (`match` with reference binding) similar to the standard library's `assert_eq!`:

```rust
#[macro_export]
macro_rules! assert_text_eq {
    ($left: expr, $right: expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                if left_val != right_val {
                    $crate::print_diff_github_style(right_val, left_val);
                    panic!("assertion failed");
                }
            }
        }
    };
}
```
This guarantees that each expression is evaluated exactly once and avoids moving ownership of the inputs.

---

## 3. Support for Custom Panic Messages (API Design & Usability)

### Description
Standard Rust assertion macros (`assert!`, `assert_eq!`, `assert_ne!`) support format arguments, e.g., `assert_eq!(a, b, "Value mismatch at index {}", i)`. Currently, `assert-text` macros do not support formatting arguments and only panic with a generic `"assertion failed"`.

### Proposed Solution
Overload the macro definitions to accept custom panic messages, allowing drop-in compatibility with standard Rust tests:

```rust
#[macro_export]
macro_rules! assert_text_eq {
    ($left: expr, $right: expr $(,)?) => {
        // ... base case
    };
    ($left: expr, $right: expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                if left_val != right_val {
                    $crate::print_diff_github_style(right_val, left_val);
                    panic!($($arg)+);
                }
            }
        }
    };
}
```

---

## 4. Redundant Capacity Allocations (Performance Optimization)

### Description
In `format_diff_line_same` and `format_diff_line_mark`, `s.reserve(line.len() + 2)` is called inside the loop:

```rust
let mut s = String::with_capacity(y.len() + 2);
for line in y.split_terminator('\n') {
    s.reserve(line.len() + 2); // Redundant and inefficient
    s.push(' ');
    s.push_str(line);
    s.push('\n');
}
```
Since `y.len() + 2` is already allocated upfront, `s.reserve(line.len() + 2)` will do nothing but perform redundant checks inside the loop. In addition, `y.len() + 2` is too small for `format_diff_line_mark` because the color escape sequences (`color_start`, `color_end`) are appended to every line, causing multiple reallocations during the loop.

### Proposed Solution
1. Remove `s.reserve(line.len() + 2)` from inside the loops.
2. Provide a better initial capacity. For example, in `format_diff_line_mark`:
   ```rust
   // Estimate 12 additional bytes per line for ANSI escape sequences and marks
   let estimated_extra = y.split_terminator('\n').count() * 12;
   let mut s = String::with_capacity(y.len() + estimated_extra);
   ```

---

## 5. Inefficient Small Allocations in `format_diff_add_rem` (Performance Optimization)

### Description
`format_diff_add_rem` constructs a `Vec<(Cattr, String)>` to keep track of words and newlines. Inside the loop, it performs a massive number of small heap allocations:
- `line.to_string()`
- `"\n".to_string()`
- `mark.to_string()`
- `" ".to_string()`

For a reasonably large text diff, this will trigger thousands of tiny allocations, leading to CPU cache misses and overhead.

### Proposed Solution
Avoid converting static markers, spaces, and newlines to `String`. You can use a structured type that holds either a borrowed `&str` or an enum with reference lifetimes:

```rust
enum DiffPart<'a> {
    Borrowed(&'a str),
    Static(&'static str),
    Mark,
}
```
This allows `ca_v` to be defined as `Vec<(Cattr, DiffPart<'a>)>`, eliminating almost all heap allocations for separators and static parts.

---

## 6. Hardcoded ANSI Colors vs the `NO_COLOR` Standard (UX / Environment Support)

### Description
The colors are hardcoded ANSI codes (`\x1b[32m`, etc.) and printed directly. This is beautiful in a terminal, but:
- It produces hard-to-read escape sequences in environments that do not support colors (e.g., non-interactive logs, text file outputs, or CI runners).
- It violates the widely accepted `NO_COLOR` standard (https://no-color.org/).

### Proposed Solution
Check if the `NO_COLOR` environment variable is present, or if stdout is not a TTY (using standard check or a crate like `is-terminal`), and strip or bypass the ANSI colors accordingly:

```rust
let use_color = std::env::var("NO_COLOR").is_err();
let color_green = if use_color { "\x1b[32m" } else { "" };
```

---
Review Date: 2026-05-26
Reviewer: Gemini CLI Agent
