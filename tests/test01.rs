//
macro_rules! txt {
    (L) => {
        "I have seldom heard him mention her under any other name.
In his eyes she eclipses and predominates the whole of her sex.
It was not that he felt any emotion."
    };
    (R) => {
        "i have seldom heard her mention his under any other name.
In here eyes she eclipses and predominates the whole of his sex.
It was not that her felt any emotion."
    };
}

// test assert_text_eq!(txt1, txt2)
#[cfg(test)]
mod test_1 {
    use assert_text::assert_text_eq;
    //
    #[test]
    fn test_11() {
        assert_text_eq!(txt!(L), txt!(L));
    }
    #[test]
    fn test_12() {
        assert_text_eq!(txt!(R), txt!(R));
    }
    #[test]
    #[should_panic = "assertion failed"]
    fn test_13() {
        assert_text_eq!(txt!(L), txt!(R));
    }
    #[test]
    #[should_panic = "custom message: foo"]
    fn test_14() {
        assert_text_eq!(txt!(L), txt!(R), "custom message: {}", "foo");
    }
}

// test assert_text_starts_with!(txt1, txt2)
#[cfg(test)]
mod test_2 {
    use assert_text::assert_text_starts_with;
    //
    #[test]
    fn test_21() {
        assert_text_starts_with!(txt!(L), "I have seldom heard him mention her under");
    }
    #[test]
    fn test_22() {
        assert_text_starts_with!(txt!(R), "i have seldom heard her mention his under");
    }
    #[test]
    #[should_panic = "assertion failed"]
    fn test_23() {
        assert_text_starts_with!(txt!(L), "i have seldom heard here");
    }
    #[test]
    fn test_utf8_starts_with_success() {
        assert_text_starts_with!("こんにちは世界", "こんにちは");
    }
    #[test]
    #[should_panic = "assertion failed"]
    fn test_utf8_starts_with_fail_panic_safe() {
        assert_text_starts_with!("こんにちは", "a");
    }
    #[test]
    #[should_panic = "custom message: bar"]
    fn test_starts_with_custom_message() {
        assert_text_starts_with!(txt!(L), "i have seldom", "custom message: {}", "bar");
    }
}

// test assert_text_ends_with!(txt1, txt2)
#[cfg(test)]
mod test_3 {
    use assert_text::assert_text_ends_with;
    //
    #[test]
    fn test_31() {
        assert_text_ends_with!(txt!(L), "not that he felt any emotion.");
    }
    #[test]
    fn test_32() {
        assert_text_ends_with!(txt!(R), "not that her felt any emotion.");
    }
    #[test]
    #[should_panic = "assertion failed"]
    fn test_33() {
        assert_text_ends_with!(txt!(L), "i have seldom heard here");
    }
    #[test]
    fn test_utf8_ends_with_success() {
        assert_text_ends_with!("こんにちは世界", "世界");
    }
    #[test]
    #[should_panic = "assertion failed"]
    fn test_utf8_ends_with_fail_panic_safe() {
        assert_text_ends_with!("こんにちは", "a");
    }
    #[test]
    #[should_panic = "custom message: baz"]
    fn test_ends_with_custom_message() {
        assert_text_ends_with!(txt!(L), "heard here", "custom message: {}", "baz");
    }
}

// test assert_text_match!(txt1, re_txt2)
#[cfg(test)]
mod test_4 {
    use assert_text::assert_text_match;
    //
    #[test]
    fn test_41() {
        assert_text_match!(txt!(L), "not that he felt any emotion.");
    }
    #[test]
    fn test_42() {
        assert_text_match!(txt!(R), "not that her felt any emotion.");
    }
    #[test]
    #[should_panic = "assertion failed"]
    fn test_43() {
        assert_text_match!(txt!(L), "i have .+ heard here");
    }
    #[test]
    #[should_panic = "custom message: qux"]
    fn test_match_custom_message() {
        assert_text_match!(txt!(L), "i have .+ heard here", "custom message: {}", "qux");
    }
}

// test assert_text_contains!(txt1, re_txt2)
#[cfg(test)]
mod test_5 {
    use assert_text::assert_text_contains;
    //
    #[test]
    fn test_51() {
        assert_text_contains!(txt!(L), "not that he felt any emotion.");
    }
    #[test]
    fn test_52() {
        assert_text_contains!(txt!(R), "not that her felt any emotion.");
    }
    #[test]
    #[should_panic = "assertion failed"]
    fn test_53() {
        assert_text_contains!(txt!(L), "i have seldom heard here");
    }
    #[test]
    #[should_panic = "custom message: quux"]
    fn test_contains_custom_message() {
        assert_text_contains!(txt!(L), "i have seldom heard here", "custom message: {}", "quux");
    }
}
