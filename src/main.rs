use std::env;
use std::fs;

mod ast;
mod cli;
mod codegen;
mod parse;

use ast::Immediate;

// TODO
// [ ] Add call to SetConsoleMode with ENABLE_VIRTUAL_TERMINAL_INPUT arg

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = cli::Parser::new(args.clone());
    match cli.parse() {
        Ok(cmd) => match cmd {
            cli::Command::PrintUsage => {
                cli.print_usage();
            }
            cli::Command::Compile(source) => {
                let compilation_result = compile_source(&source);
                if let Err(error) = compilation_result {
                    println_err(&error);
                    std::process::exit(1);
                }
            }
        },
        Err(error) => match error {
            cli::Error::UnknownCommand(cmd) => {
                println!("error: unknown command \"{}\"", cmd);
                cli.print_help();
            }
            cli::Error::NotAFile(_) => {
                todo!()
            }
            cli::Error::MissingTextArg => {
                println!("error: missing argument to --text");
                cli.print_help();
            }
        },
    }
}

fn compile_source(source: &str) -> Result<(), String> {
    let parse_result = parse::parse_immediate_value(source);
    if let None = parse_result {
        return Err(format!("couldn't parse \"{}\"", source));
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

    Ok(())
}

/// prints `err` prefixed with a bold red "error:"
fn println_err(err: &str) {
    println!("\x1b[31;1merror\x1b[97;1m:\x1b[0m {}", err);
}
