/*!
the testing macro tools.

This checks that strings are equal.
You will see different characters if that is different.

# Features

- assert_text_eq!(txt1, txt2)
- assert_text_contains!(txt1, txt2)
- assert_text_starts_with!(txt1, txt2)
- assert_text_ends_with!(txt1, txt2)
- assert_text_match!(txt1, regex_text2)
- supports custom panic messages
- minimum support rustc 1.65.0 (897e37553 2022-11-02)

*/

/// Asserts that two text expressions are equal.
///
/// If the texts are not equal, it prints a GitHub-style diff and panics.
///
/// # Arguments
///
/// * `$left` - The first text expression.
/// * `$right` - The second text expression.
///
/// # Examples
///
/// ```
/// use assert_text::assert_text_eq;
/// assert_text_eq!("hello", "hello");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_eq;
/// assert_text_eq!("hello", "world");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_eq;
/// assert_text_eq!("hello", "world", "custom message: {}", "foo");
/// ```
#[macro_export]
macro_rules! assert_text_eq {
    ($left: expr, $right: expr $(,)?) => {
        $crate::assert_text_eq!($left, $right, "assertion failed")
    };
    ($left: expr, $right: expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                if left_val != right_val {
                    $crate::print_diff_github_style(right_val, left_val);
                    panic!($($arg)+)
                }
            }
        }
    };
}

/// Asserts that the first text expression starts with the second text expression.
///
/// If the first text does not start with the second, it prints a GitHub-style diff
/// of the differing prefix and panics.
///
/// # Arguments
///
/// * `$left` - The text expression to check.
/// * `$right` - The prefix to check against.
///
/// # Examples
///
/// ```
/// use assert_text::assert_text_starts_with;
/// assert_text_starts_with!("hello world", "hello ");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_starts_with;
/// assert_text_starts_with!("hello world", "goodbye");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_starts_with;
/// assert_text_starts_with!("hello world", "goodbye", "custom message: {}", "foo");
/// ```
#[macro_export]
macro_rules! assert_text_starts_with {
    ($left: expr, $right: expr $(,)?) => {
        $crate::assert_text_starts_with!($left, $right, "assertion failed")
    };
    ($left: expr, $right: expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                if !left_val.starts_with(right_val) {
                    let right_chars = right_val.chars().count();
                    let limit = left_val
                        .char_indices()
                        .nth(right_chars)
                        .map(|(idx, _)| idx)
                        .unwrap_or_else(|| left_val.len());
                    let edit = &left_val[..limit];
                    $crate::print_diff_github_style(right_val, edit);
                    panic!($($arg)+)
                }
            }
        }
    };
}

/// Asserts that the first text expression ends with the second text expression.
///
/// If the first text does not end with the second, it prints a GitHub-style diff
/// of the differing suffix and panics.
///
/// # Arguments
///
/// * `$left` - The text expression to check.
/// * `$right` - The suffix to check against.
///
/// # Examples
///
/// ```
/// use assert_text::assert_text_ends_with;
/// assert_text_ends_with!("hello world", " world");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_ends_with;
/// assert_text_ends_with!("hello world", "goodbye");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_ends_with;
/// assert_text_ends_with!("hello world", "goodbye", "custom message: {}", "foo");
/// ```
#[macro_export]
macro_rules! assert_text_ends_with {
    ($left: expr, $right: expr $(,)?) => {
        $crate::assert_text_ends_with!($left, $right, "assertion failed")
    };
    ($left: expr, $right: expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                if !left_val.ends_with(right_val) {
                    let right_chars = right_val.chars().count();
                    let total_chars = left_val.chars().count();
                    let skip_chars = total_chars.saturating_sub(right_chars);
                    let limit = left_val
                        .char_indices()
                        .nth(skip_chars)
                        .map(|(idx, _)| idx)
                        .unwrap_or(0);
                    let edit = &left_val[limit..];
                    $crate::print_diff_github_style(right_val, edit);
                    panic!($($arg)+)
                }
            }
        }
    };
}

/// Asserts that the first text contains the given second text.
///
/// If the text does not contains second text, it panics.
///
/// # Arguments
///
/// * `$left` - The text expression to check.
/// * `$right` - The second text expression.
///
/// # Examples
///
/// ```
/// use assert_text::assert_text_contains;
/// assert_text_contains!("hello world", "o w");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_contains;
/// assert_text_contains!("hello world", "apple");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_contains;
/// assert_text_contains!("hello world", "apple", "custom message: {}", "foo");
/// ```
#[macro_export]
macro_rules! assert_text_contains {
    ($left: expr, $right: expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                if !left_val.contains(right_val) {
                    $crate::assert_text_contains!(
                        left_val,
                        right_val,
                        concat!("assertion failed\n", "  left: \"{}\"\n", " right: \"{}\""),
                        left_val.escape_debug(),
                        right_val.escape_debug(),
                    )
                }
            }
        }
    };
    ($left: expr, $right: expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                if !left_val.contains(right_val) {
                    panic!($($arg)+);
                }
            }
        }
    };
}

/// Asserts that the first text expression matches the given regular expression.
///
/// If the text does not match the regex, it panics.
///
/// # Arguments
///
/// * `$left` - The text expression to check.
/// * `$right` - The regular expression string.
///
/// # Panics
///
/// Panics if the `$right` string is not a valid regular expression.
///
/// # Examples
///
/// ```
/// use assert_text::assert_text_match;
/// assert_text_match!("hello world", r"^h.+d$");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_match;
/// assert_text_match!("hello world", r"^goodbye.*");
/// ```
///
/// ```should_panic
/// use assert_text::assert_text_match;
/// assert_text_match!("hello world", r"^goodbye.*", "custom message: {}", "foo");
/// ```
#[macro_export]
macro_rules! assert_text_match {
    ($left: expr, $right: expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                let re = regex::Regex::new(right_val).unwrap();
                if !re.is_match(left_val) {
                    $crate::assert_text_match!(
                        left_val,
                        right_val,
                        concat!("assertion failed\n", "  left: \"{}\"\n", " regex: \"{}\""),
                        left_val.escape_debug(),
                        right_val.escape_debug(),
                    )
                }
            }
        }
    };
    ($left: expr, $right: expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_val: &str = left_val.as_ref();
                let right_val: &str = right_val.as_ref();
                let re = regex::Regex::new(right_val).unwrap();
                if !re.is_match(left_val) {
                    panic!($($arg)+);
                }
            }
        }
    };
}

use difference::{Changeset, Difference};
use std::string::ToString;

/// Prints a GitHub-style diff between two text slices to stdout.
///
/// This function highlights additions in green and removals in red.
///
/// # Arguments
///
/// * `text1` - The original text.
/// * `text2` - The modified text.
///
/// # Examples
///
/// ```
/// use assert_text::print_diff_github_style;
/// print_diff_github_style("hello world", "Hello orld");
/// ```
pub fn print_diff_github_style(text1: &str, text2: &str) {
    //
    let color_green = "\x1b[32m";
    let color_red = "\x1b[31m";
    let color_bright_green = "\x1b[1;32m";
    let color_reverse_red = "\x1b[31;7m";
    let color_reverse_green = "\x1b[32;7m";
    let color_end = "\x1b[0m";
    //
    let mut out_s = String::new();
    //
    let Changeset { diffs, .. } = Changeset::new(text1, text2, "\n");
    //
    for i in 0..diffs.len() {
        let s = match diffs[i] {
            Difference::Same(ref y) => format_diff_line_same(y),
            Difference::Add(ref y) => {
                let opt = if i > 0 {
                    if let Difference::Rem(ref x) = diffs[i - 1] {
                        Some(format_diff_add_rem(
                            "+",
                            x,
                            y,
                            color_green,
                            color_reverse_green,
                            color_end,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                };
                match opt {
                    Some(a) => a,
                    None => format_diff_line_mark("+", y, color_bright_green, color_end),
                }
            }
            Difference::Rem(ref y) => {
                let opt = if i < diffs.len() - 1 {
                    if let Difference::Add(ref x) = diffs[i + 1] {
                        Some(format_diff_add_rem(
                            "-",
                            x,
                            y,
                            color_red,
                            color_reverse_red,
                            color_end,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                };
                match opt {
                    Some(a) => a,
                    None => format_diff_line_mark("-", y, color_red, color_end),
                }
            }
        };
        out_s.push_str(s.as_str());
    }
    //
    print!("{}", out_s.as_str());
}

/// Formats a line that is the same in both texts for diff output.
/// Prepends a space to the line.
#[inline(never)]
fn format_diff_line_same(y: &str) -> String {
    let mut s = String::with_capacity(y.len() + 2);
    for line in y.split_terminator('\n') {
        s.reserve(line.len() + 2);
        s.push(' ');
        s.push_str(line);
        s.push('\n');
    }
    s
}

/// Formats a line that is either added or removed, with a specific mark and color.
#[inline(never)]
fn format_diff_line_mark(
    mark: &str, // "+" or "-"
    y: &str,
    color_start: &str,
    color_end: &str,
) -> String {
    let mut s = String::with_capacity(y.len() + 2);
    for line in y.split_terminator('\n') {
        s.reserve(line.len() + 2);
        s.push_str(color_start);
        s.push_str(mark);
        s.push_str(line);
        s.push_str(color_end);
        s.push('\n');
    }
    s
}

/// Formats a line that has been changed (both added and removed parts) for diff output.
#[inline(never)]
fn format_diff_add_rem(
    mark: &str, // "+" or "-"
    x: &str,
    y: &str,
    color_fore: &str,
    color_reverse: &str,
    color_end: &str,
) -> String {
    //
    #[derive(PartialEq, Copy, Clone)]
    enum Cattr {
        None,
        Fore,
        Reve,
    }
    //
    let mut ca_v: Vec<(Cattr, String)> = vec![(Cattr::Fore, mark.to_string())];
    //
    let Changeset { diffs, .. } = Changeset::new(x, y, " ");
    for c in diffs {
        match c {
            Difference::Same(ref z) => {
                for line in z.split_terminator('\n') {
                    ca_v.push((Cattr::Fore, line.to_string()));
                    ca_v.push((Cattr::None, "\n".to_string()));
                    ca_v.push((Cattr::Fore, mark.to_string()));
                }
                let bytes = z.as_bytes();
                let len = bytes.len();
                if len >= 1 && bytes[len - 1] != b'\n' {
                    ca_v.pop();
                    ca_v.pop();
                }
                ca_v.push((Cattr::Fore, " ".to_string()));
            }
            Difference::Add(ref z) => {
                for line in z.split_terminator('\n') {
                    ca_v.push((Cattr::Reve, line.to_string()));
                    ca_v.push((Cattr::None, "\n".to_string()));
                    ca_v.push((Cattr::Fore, mark.to_string()));
                }
                let bytes = z.as_bytes();
                let len = bytes.len();
                if len >= 1 && bytes[len - 1] != b'\n' {
                    ca_v.pop();
                    ca_v.pop();
                }
                ca_v.push((Cattr::Fore, " ".to_string()));
            }
            _ => {}
        };
    }
    //
    let mut out_s = String::with_capacity(x.len().max(y.len()) * 2);
    let mut prev_a: Cattr = Cattr::None;
    for (cat, st) in &ca_v {
        //
        if prev_a != *cat {
            if prev_a != Cattr::None {
                out_s.push_str(color_end)
            }
            if *cat == Cattr::Fore {
                out_s.push_str(color_fore);
            } else if *cat == Cattr::Reve {
                out_s.push_str(color_reverse);
            }
            prev_a = *cat;
        }
        out_s.push_str(st.as_str());
    }
    if prev_a != Cattr::None {
        out_s.push_str(color_end);
    }
    out_s.push('\n');
    //
    out_s
}
