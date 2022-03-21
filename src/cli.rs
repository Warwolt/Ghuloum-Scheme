use std::fs;

pub enum Command {
    PrintUsage,
    Compile(String),
}

pub enum Error {
    UnknownCommand(String),
    NotAFile(String),
    MissingTextArg,
}

pub struct Parser {
    args: Vec<String>,
}

impl Parser {
    pub fn new(args: Vec<String>) -> Self {
        Parser { args }
    }

    pub fn parse(&self) -> Result<Command, Error> {
        if self.args.len() < 2 {
            return Ok(Command::PrintUsage);
        }

        if ["-h", "--help"].contains(&self.args[1].as_ref()) {
            return Ok(Command::PrintUsage);
        }

        if &self.args[1] == "--src" {
            if self.args.len() < 3 {
                return Err(Error::MissingTextArg);
            }
            return Ok(Command::Compile(self.args[2].clone()));
        }

        if &self.args[1][..2] == "--" {
            return Err(Error::UnknownCommand(self.args[1].clone()));
        }

        let file = fs::read(&self.args[1]);
        if let Err(_) = file {
            return Err(Error::NotAFile(self.args[1].clone()));
        }
        let source = String::from_utf8(file.unwrap()).expect("file contained non-utf8 content");

        Ok(Command::Compile(String::from(source.trim())))
    }

    pub fn print_usage(&self) {
        let exe_name = std::path::Path::new(&self.args[0]).file_name().unwrap();
        println!(
            "usage: {} [<source-file>] [-h | --help] [--text <source-text>]",
            exe_name.to_str().unwrap()
        );
    }

    pub fn print_help(&self) {
        self.print_usage();
        println!("");
        println!("<source-file>             compiles <source-file>");
        println!("-h | --help               prints this help message");
        println!("--src <source-text>       directly compiles the argument <source-text>");
    }
}
