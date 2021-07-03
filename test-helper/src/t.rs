use assert_text;

pub fn t0(txt1: &str, txt2: &str) {
    assert_text::print_diff_github_style(txt2, txt1);
}

pub fn t1(txt1: &str, txt2: &str) {
    assert_text_eq!(txt1, txt2);
}

pub fn t2(txt1: &str, txt2: &str) {
    assert_text_starts_with!(txt1, txt2);
}

pub fn t3(txt1: &str, txt2: &str) {
    assert_text_ends_with!(txt1, txt2);
}
