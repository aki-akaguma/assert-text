const TARGET_EXE_PATH: &str = env!("CARGO_BIN_EXE_test-helper");

macro_rules! fixtures {
    ($n: expr) => {
        concat!("fixtures/", $n)
    };
}

#[allow(unused_macros)]
#[rustversion::since(1.42.0)]
macro_rules! note_backtrace {
    () => {
        "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n"
    };
}

#[rustversion::before(1.42.0)]
macro_rules! note_backtrace {
    () => {
        "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n"
    };
}

#[cfg(not(windows))]
macro_rules! ds {
    () => {
        "/"
    };
}

#[cfg(windows)]
#[rustversion::since(1.91.0)]
macro_rules! ds {
    () => {
        "\\\\"
    };
}

#[cfg(windows)]
#[rustversion::before(1.91.0)]
macro_rules! ds {
    () => {
        "\\"
    };
}

#[rustversion::since(1.91.0)]
macro_rules! thread_panic_error_out_base {
    ($rowcol:expr) => {
        concat!(
            "\nthread \'main\' \\(\\d+\\) panicked at ",
            concat!("test-helper", ds!(), "src", ds!(), "t.rs"),
            ":",
            $rowcol,
            ":\nassertion failed\n",
            note_backtrace!()
        )
    };
}

#[rustversion::all(since(1.85.0),before(1.91.0))]
macro_rules! thread_panic_error_out_base {
    ($rowcol:expr) => {
        concat!(
            "\nthread \'main\' panicked at ",
            concat!("test-helper", ds!(), "src", ds!(), "t.rs"),
            ":",
            $rowcol,
            ":\nassertion failed\n",
            note_backtrace!()
        )
    };
}

#[rustversion::all(since(1.73.0),before(1.85.0))]
macro_rules! thread_panic_error_out_base {
    ($rowcol:expr) => {
        concat!(
            "thread \'main\' panicked at ",
            concat!("test-helper", ds!(), "src", ds!(), "t.rs"),
            ":",
            $rowcol,
            ":\nassertion failed\n",
            note_backtrace!()
        )
    };
}

#[rustversion::before(1.73.0)]
macro_rules! thread_panic_error_out_base {
    ($rowcol:expr) => {
        concat!(
            "thread \'main\' panicked at \'assertion failed\', ",
            concat!("test-helper", ds!(), "src", ds!(), "t.rs"),
            ":",
            $rowcol,
            "\n",
            note_backtrace!()
        )
    };
}

#[rustversion::since(1.91.0)]
macro_rules! assert_eq_thread_panic_error_out {
    ($errout:expr) => {{
        assert!(
            regex::Regex::new(thread_panic_error_out!())
                .unwrap()
                .is_match($errout.as_str()),
            "*errout: {}, *reg: {},*",
            $errout.as_str(),
            thread_panic_error_out!()
        );
    }};
}

#[rustversion::before(1.91.0)]
macro_rules! assert_eq_thread_panic_error_out {
    ($errout:expr) => {
        assert_eq!($errout, thread_panic_error_out!());
    };
}

// test assert_text_eq!(txt1, txt2)
mod test_1 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    macro_rules! thread_panic_error_out {
        () => {
            thread_panic_error_out_base!("8:5")
        };
    }
    //
    #[test]
    fn test_11() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["1", fixtures!("text1.txt"), fixtures!("text1.txt")],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    #[test]
    fn test_12() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["1", fixtures!("text1.txt"), fixtures!("text2.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(
            oup.stdout,
            concat!(
                " How razorback-jumping frogs can level six piqued gymnasts!\n",
                " Cozy lummox gives smart squid who asks for job pen.\n",
                "\u{1b}[1;32m+Adjusting quiver and bow, Zompyc killed the fox.\u{1b}[0m\n",
                " The quick onyx goblin jumps over the lazy dwarf.\n",
                " My faxed joke won a pager in the cable TV quiz show.\n",
                "\u{1b}[31m-The quick brown fox jumps over the \u{1b}[0m\u{1b}[31;7ma\u{1b}[0m\u{1b}[31m dog. \u{1b}[0m\n",
                "\u{1b}[32m+The quick brown fox jumps over the \u{1b}[0m\u{1b}[32;7mlazy\u{1b}[0m\u{1b}[32m dog. \u{1b}[0m\n"
            )
        );
        assert!(!oup.status.success());
    }
    #[test]
    fn test_13() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["1", fixtures!("text1.txt"), fixtures!("text3.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(
            oup.stdout,
            concat!(
                " How razorback-jumping frogs can level six piqued gymnasts!\n",
                " Cozy lummox gives smart squid who asks for job pen.\n",
                "\u{1b}[1;32m+Adjusting quiver and bow, Zompyc killed the fox.\u{1b}[0m\n",
                "\u{1b}[1;32m+The quick onyx goblin jumps over the lazy dwarf.\u{1b}[0m\n",
                "\u{1b}[1;32m+My faxed joke won a pager in the cable TV quiz show.\u{1b}[0m\n",
                "\u{1b}[1;32m+The quick brown fox jumps over the lazy dog.\u{1b}[0m\n"
            )
        );
        assert!(!oup.status.success());
    }
    #[test]
    fn test_14() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["1", fixtures!("text1.txt"), fixtures!("text4.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(
            oup.stdout,
            concat!(
                "\u{1b}[1;32m+How razorback-jumping frogs can level six piqued gymnasts!\u{1b}[0m\n",
                "\u{1b}[1;32m+Cozy lummox gives smart squid who asks for job pen.\u{1b}[0m\n",
                "\u{1b}[1;32m+Adjusting quiver and bow, Zompyc killed the fox.\u{1b}[0m\n",
                "\u{1b}[1;32m+The quick onyx goblin jumps over the lazy dwarf.\u{1b}[0m\n",
                " My faxed joke won a pager in the cable TV quiz show.\n",
                " The quick brown fox jumps over the lazy dog.\n"
            )
        );
        assert!(!oup.status.success());
    }
    #[test]
    fn test_15() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["1", fixtures!("text1.txt"), fixtures!("text5.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(
            oup.stdout,
            concat!(
                " How razorback-jumping frogs can level six piqued gymnasts!\n",
                " Cozy lummox gives smart squid who asks for job pen.\n",
                "\u{1b}[31m-Adjusting \u{1b}[0m\u{1b}[31;7mthe\u{1b}[0m\u{1b}[31m quiver and \u{1b}[0m\u{1b}[31;7mthe\u{1b}[0m\u{1b}[31m bow, Zompyc killed the fox.\u{1b}[0m\n",
                "\u{1b}[31m-The quick onyx goblin jumps over \u{1b}[0m\u{1b}[31;7ma loose\u{1b}[0m\u{1b}[31m dwarf. \u{1b}[0m\n",
                "\u{1b}[32m+Adjusting quiver and bow, Zompyc killed the fox.\u{1b}[0m\n",
                "\u{1b}[32m+The quick onyx goblin jumps over \u{1b}[0m\u{1b}[32;7mthe lazy\u{1b}[0m\u{1b}[32m dwarf. \u{1b}[0m\n",
                " My faxed joke won a pager in the cable TV quiz show.\n",
                " The quick brown fox jumps over the lazy dog.\n"
            )
        );
        assert!(!oup.status.success());
    }
}

// test assert_text_starts_with!(txt1, txt2)
mod test_2 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    macro_rules! thread_panic_error_out {
        () => {
            thread_panic_error_out_base!("12:5")
        };
    }
    //
    #[test]
    fn test_11() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["2", fixtures!("text1.txt"), fixtures!("text1.txt")],
        );
        assert!(oup.status.success());
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.stderr, "");
    }
    #[test]
    fn test_12() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["2", fixtures!("text1.txt"), fixtures!("text2.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(
            oup.stdout,
            concat!(
                " How razorback-jumping frogs can level six piqued gymnasts!\n",
                " Cozy lummox gives smart squid who asks for job pen.\n",
                "\u{1b}[1;32m+Adjusting quiver and bow, Zompyc killed the fox.\u{1b}[0m\n",
                " The quick onyx goblin jumps over the lazy dwarf.\n",
                "\u{1b}[31m-My faxed joke won a pager in the cable TV quiz \u{1b}[0m\u{1b}[31;7mshow.\u{1b}[0m\n\u{1b}[31m-\u{1b}[0m\u{1b}[31;7mThe quick brown fox jumps over the a dog.\u{1b}[0m\n",
                "\u{1b}[31m- \u{1b}[0m\n\u{1b}[32m+My faxed joke won a pager in the cable TV quiz \u{1b}[0m\n"
            )
        );
        assert!(!oup.status.success());
    }
    #[test]
    fn test_13() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["2", fixtures!("text1.txt"), fixtures!("text3.txt")],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    #[test]
    fn test_14() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["2", fixtures!("text1.txt"), fixtures!("text4.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(
            oup.stdout,
            concat!(
                "\u{1b}[31m-\u{1b}[0m\u{1b}[31;7mMy faxed joke won a pager in the cable TV quiz show.\u{1b}[0m\n",
                "\u{1b}[31m-\u{1b}[0m\u{1b}[31;7mThe quick brown fox jumps over the lazy dog.\u{1b}[0m\n",
                "\u{1b}[31m- \u{1b}[0m\n\u{1b}[32m+\u{1b}[0m\u{1b}[32;7mHow razorback-jumping frogs can level six piqued gymnasts!\u{1b}[0m\n",
                "\u{1b}[32m+\u{1b}[0m\u{1b}[32;7mCozy lummox gives smart squid who asks \u{1b}[0m\u{1b}[32m \u{1b}[0m\n"
            )
        );
        assert!(!oup.status.success());
    }
    #[test]
    fn test_15() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["2", fixtures!("text1.txt"), fixtures!("text5.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(
            oup.stdout,
            concat!(
                " How razorback-jumping frogs can level six piqued gymnasts!\n",
                " Cozy lummox gives smart squid who asks for job pen.\n",
                "\u{1b}[31m-Adjusting \u{1b}[0m\u{1b}[31;7mthe\u{1b}[0m\u{1b}[31m quiver and \u{1b}[0m\u{1b}[31;7mthe\u{1b}[0m\u{1b}[31m bow, Zompyc killed the fox.\u{1b}[0m\n",
                "\u{1b}[31m-The quick onyx goblin jumps over \u{1b}[0m\u{1b}[31;7ma loose\u{1b}[0m\u{1b}[31m dwarf. \u{1b}[0m\n",
                "\u{1b}[32m+Adjusting quiver and bow, Zompyc killed the fox.\u{1b}[0m\n",
                "\u{1b}[32m+The quick onyx goblin jumps over \u{1b}[0m\u{1b}[32;7mthe lazy\u{1b}[0m\u{1b}[32m dwarf. \u{1b}[0m\n",
                " My faxed joke won a pager in the cable TV quiz show.\n",
                " The quick brown fox jumps over the lazy dog.\n"
            )
        );
        assert!(!oup.status.success());
    }
}

// test assert_text_ends_with!(txt1, txt2)
mod test_3 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    macro_rules! thread_panic_error_out {
        () => {
            thread_panic_error_out_base!("16:5")
        };
    }
    //
    #[test]
    fn test_11() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["3", fixtures!("text1.txt"), fixtures!("text1.txt")],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    #[test]
    fn test_12() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["3", fixtures!("text1.txt"), fixtures!("text2.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(oup.stdout, concat!(
            "\u{1b}[31m-\u{1b}[0m\u{1b}[31;7mHow razorback-jumping frogs can level six piqued gymnasts!\u{1b}[0m\u{1b}[31m \u{1b}[0m\n\u{1b}[32m+\u{1b}[0m\u{1b}[32;7mnasts!\u{1b}[0m\u{1b}[32m \u{1b}[0m\n",
            " Cozy lummox gives smart squid who asks for job pen.\n",
            "\u{1b}[1;32m+Adjusting quiver and bow, Zompyc killed the fox.\u{1b}[0m\n",
            " The quick onyx goblin jumps over the lazy dwarf.\n",
            " My faxed joke won a pager in the cable TV quiz show.\n",
            "\u{1b}[31m-The quick brown fox jumps over the \u{1b}[0m\u{1b}[31;7ma\u{1b}[0m\u{1b}[31m dog. \u{1b}[0m\n\u{1b}[32m+The quick brown fox jumps over the \u{1b}[0m\u{1b}[32;7mlazy\u{1b}[0m\u{1b}[32m dog. \u{1b}[0m\n"
            ));
        assert!(!oup.status.success());
    }
    #[test]
    fn test_13() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["3", fixtures!("text1.txt"), fixtures!("text3.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(oup.stdout, concat!(
            "\u{1b}[31m-\u{1b}[0m\u{1b}[31;7mHow razorback-jumping frogs can level six piqued gymnasts!\u{1b}[0m\n",
            "\u{1b}[31m-\u{1b}[0m\u{1b}[31;7mCozy lummox gives smart squid who asks for job pen.\u{1b}[0m\n",
            "\u{1b}[31m- \u{1b}[0m\n\u{1b}[32m+\u{1b}[0m\u{1b}[32;7m lazy dwarf.\u{1b}[0m\n\u{1b}[32m+\u{1b}[0m\u{1b}[32;7mMy faxed joke won a pager in the cable TV quiz show.\u{1b}[0m\n",
            "\u{1b}[32m+\u{1b}[0m\u{1b}[32;7mThe quick brown fox jumps over the lazy dog.\u{1b}[0m\n",
            "\u{1b}[32m+ \u{1b}[0m\n"
            ));
        assert!(!oup.status.success());
    }
    #[test]
    fn test_14() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["3", fixtures!("text1.txt"), fixtures!("text4.txt")],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    #[test]
    fn test_15() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["3", fixtures!("text1.txt"), fixtures!("text5.txt")],
        );
        assert_eq_thread_panic_error_out!(oup.stderr);
        assert_eq!(
            oup.stdout,
            concat!(
                " How razorback-jumping frogs can level six piqued gymnasts!\n",
                " Cozy lummox gives smart squid who asks for job pen.\n",
                "\u{1b}[31m-Adjusting \u{1b}[0m\u{1b}[31;7mthe\u{1b}[0m\u{1b}[31m quiver and \u{1b}[0m\u{1b}[31;7mthe\u{1b}[0m\u{1b}[31m bow, Zompyc killed the fox.\u{1b}[0m\n",
                "\u{1b}[31m-The quick onyx goblin jumps over \u{1b}[0m\u{1b}[31;7ma loose\u{1b}[0m\u{1b}[31m dwarf. \u{1b}[0m\n",
                "\u{1b}[32m+Adjusting quiver and bow, Zompyc killed the fox.\u{1b}[0m\n",
                "\u{1b}[32m+The quick onyx goblin jumps over \u{1b}[0m\u{1b}[32;7mthe lazy\u{1b}[0m\u{1b}[32m dwarf. \u{1b}[0m\n",
                " My faxed joke won a pager in the cable TV quiz show.\n The quick brown fox jumps over the lazy dog.\n"
            )
        );
        assert!(!oup.status.success());
    }
}
