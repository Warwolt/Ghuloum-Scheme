use std::env;
use std::fs;

// TODO:
// [x] fixnum constants
// [x] boolean constants
// [x] character constants
// [x] empty list constants
// [ ] read input from source file

mod ast;
mod codegen;
mod parse;

use ast::Immediate;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let parse_result = parse::parse_immediate_value(&args);
    if let None = parse_result {
        println!("error: couldn't parse \"{}\"", args[1]);
        print_usage(&args[0]);
        std::process::exit(1);
    }
    let immediate = parse_result.unwrap();
    let constant_asm = &match immediate {
        Immediate::Boolean(val) => codegen::emit_boolean(val),
        Immediate::Character(val) => codegen::emit_character(val),
        Immediate::Number(val) => codegen::emit_fixint(val),
        Immediate::EmptyList => codegen::emit_empty_list(),
    };

    let mut program = String::new();
    program.push_str(&codegen::emit_entry_point());
    program.push_str(constant_asm);

    fs::create_dir_all("./build").expect("Unable to create build directory");
    fs::write("./build/program.s", program).expect("Unable to write file");
}

fn print_usage(arg0: &str) {
    let exe_name = std::path::Path::new(arg0).file_name().unwrap();
    println!("usage: {} <integer>|#t|#f", exe_name.to_str().unwrap());
}
