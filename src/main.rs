use std::env;
use std::fmt::Write;
use std::fs;

// TODO:
// [x] fixnum constants
// [x] boolean constants
// [x] character constants
// [x] empty list constants
// [ ] read input from source file

const BOOLEAN_TAG: i32 = 0b00011111;
const BOOLEAN_SHIFT: i32 = 7;

const CHARACTER_TAG: i32 = 0b00001111;
const CHARACTER_SHIFT: i32 = 8;

const FIXNUM_SHIFT: i32 = 2;

const EMPTY_LIST: i32 = 0b00101111;

enum Immediate {
    Boolean(bool),
    Character(i32),
    Number(i32),
    EmptyList,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let parse_result = parse_immediate_value(&args);
    if let None = parse_result {
        println!("error: couldn't parse \"{}\"", args[1]);
        print_usage(&args[0]);
        std::process::exit(1);
    }
    let immediate = parse_result.unwrap();
    let constant_asm = &match immediate {
        Immediate::Boolean(val) => emit_boolean(val),
        Immediate::Character(val) => emit_character(val),
        Immediate::Number(val) => emit_fixint(val),
        Immediate::EmptyList => emit_empty_list(),
    };

    let mut program = String::new();
    program.push_str(&emit_entry_point());
    program.push_str(constant_asm);

    fs::create_dir_all("./build").expect("Unable to create build directory");
    fs::write("./build/program.s", program).expect("Unable to write file");
}

fn parse_immediate_value(args: &Vec<String>) -> Option<Immediate> {
    let input = &args[1];

    /* Parse booleans */
    if input == "#t" {
        return Some(Immediate::Boolean(true));
    }
    if input == "#f" {
        return Some(Immediate::Boolean(false));
    }

    /* Parse fixint */
    let number_parse = input.parse::<i32>();
    if let Ok(number) = number_parse {
        return Some(Immediate::Number(number));
    }

    /* Parse character */
    if input.len() > 2 && &input[..2] == "#\\" {
        let tail = &input[2..];
        if tail.is_ascii() {
            if ["\t", "\r", "\n"].contains(&tail) {
                let c = tail.parse::<char>().unwrap();
                return Some(Immediate::Character(c as i32));
            }

            if tail.len() == 1 {
                let c = tail.chars().next().unwrap();
                return Some(Immediate::Character(c as i32));
            }
        }
    }

    /* Parse empty list */
    if input == "()" {
        return Some(Immediate::EmptyList);
    }

    None
}

fn print_usage(arg0: &str) {
    let exe_name = std::path::Path::new(arg0).file_name().unwrap();
    println!("usage: {} <integer>|#t|#f", exe_name.to_str().unwrap());
}

#[allow(unused_must_use)]
fn emit_entry_point() -> String {
    let mut text = String::new();

    writeln!(&mut text, "    .text");
    writeln!(&mut text, "    .p2align 4,,15");
    writeln!(&mut text, ".global entry_point");
    writeln!(&mut text, "entry_point:");

    text
}

#[allow(unused_must_use)]
fn emit_boolean(val: bool) -> String {
    let mut text = String::new();

    // add type information to immediate
    let shifted_val = BOOLEAN_TAG | ((val as i32) << BOOLEAN_SHIFT);

    writeln!(&mut text, "    movl    ${}, %eax", shifted_val);
    writeln!(&mut text, "    ret");

    text
}

#[allow(unused_must_use)]
fn emit_character(val: i32) -> String {
    let mut text = String::new();

    // add type information to immediate
    let shifted_val = CHARACTER_TAG | ((val as i32) << CHARACTER_SHIFT);

    writeln!(&mut text, "    movl    ${}, %eax", shifted_val);
    writeln!(&mut text, "    ret");

    text
}

#[allow(unused_must_use)]
fn emit_fixint(val: i32) -> String {
    let mut text = String::new();

    // add type information to immediate
    let shifted_val = val << FIXNUM_SHIFT;

    writeln!(&mut text, "    movl    ${}, %eax", shifted_val);
    writeln!(&mut text, "    ret");

    text
}

#[allow(unused_must_use)]
fn emit_empty_list() -> String {
    let mut text = String::new();

    writeln!(&mut text, "    movl    ${}, %eax", EMPTY_LIST);
    writeln!(&mut text, "    ret");

    text
}
