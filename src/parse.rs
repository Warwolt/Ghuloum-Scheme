use crate::ast::Immediate;

pub fn parse_immediate_value(args: &Vec<String>) -> Option<Immediate> {
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
                let c = tail.parse::<char>().unwrap() as u8;
                return Some(Immediate::Character(c));
            }

            if tail.len() == 1 {
                let c = tail.chars().next().unwrap() as u8;
                return Some(Immediate::Character(c));
            }
        }
    }

    /* Parse empty list */
    if input == "()" {
        return Some(Immediate::EmptyList);
    }

    None
}
