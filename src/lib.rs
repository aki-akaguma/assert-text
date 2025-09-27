/*!
the testing macro tools.

This checks that strings are equal.
You will see different characters if that is different.

# Features

- assert_text_eq!(txt1, txt2)
- assert_text_starts_with!(txt1, txt2)
- assert_text_ends_with!(txt1, txt2)
- assert_text_match!(txt1, regex_text2)
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
#[macro_export]
macro_rules! assert_text_eq {
    ($left: expr, $right: expr) => {
        if $left != $right {
            let orig = $right;
            let edit = &$left[0..];
            $crate::print_diff_github_style(orig, edit);
            panic!("assertion failed")
        };
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
#[macro_export]
macro_rules! assert_text_starts_with {
    ($left: expr, $right: expr) => {
        if !$left.starts_with($right) {
            let ll = $left.len();
            let rl = $right.len();
            let orig = $right;
            let edit = &$left[0..ll.min(rl)];
            $crate::print_diff_github_style(orig, edit);
            panic!("assertion failed")
        };
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
#[macro_export]
macro_rules! assert_text_ends_with {
    ($left: expr, $right: expr) => {
        if !$left.ends_with($right) {
            let ll = $left.len();
            let rl = $right.len();
            let orig = $right;
            let edit = &$left[if ll > rl { ll - rl } else { 0 }..];
            $crate::print_diff_github_style(orig, edit);
            panic!("assertion failed")
        };
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
#[macro_export]
macro_rules! assert_text_match {
    ($left: expr, $right: expr) => {
        let re = regex::Regex::new($right).unwrap();
        if !re.is_match($left) {
            panic!(concat!(
                "assertion failed\n",
                "  left: \"",
                $left,
                "\"\n",
                " regex: ",
                "\"\n",
                $right,
                "\""
            ));
        };
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
