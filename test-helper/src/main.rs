#[macro_use]
extern crate assert_text;

use std::env;
use std::fs;
use std::io::Read;

mod t;

fn main() {
    let (sw, file1, file2) = get_cmd_params();
    let txt1 = read_file(file1);
    let txt2 = read_file(file2);
    match &*sw {
        "0" => t::t0(txt1.as_str(), txt2.as_str()),
        "1" => t::t1(txt1.as_str(), txt2.as_str()),
        "2" => t::t2(txt1.as_str(), txt2.as_str()),
        "3" => t::t3(txt1.as_str(), txt2.as_str()),
        _ => panic!("sw should be 0,1,2 or 3."),
    };
}

fn get_cmd_params() -> (String, String, String) {
    let mut env_args: Vec<String> = env::args().collect();
    let _program = env_args.remove(0);
    //
    if env_args.len() < 3 {
        panic!("should have 3 argument. [cmd] sw file1 file2");
    }
    let sw = env_args[0].clone();
    let file1 = env_args[1].clone();
    let file2 = env_args[2].clone();
    (sw, file1, file2)
}

fn read_file(file1: String) -> String {
    let mut s = String::new();
    fs::File::open(file1)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    #[cfg(windows)]
    let s = s.replace("\r\n", "\n");
    s
}
