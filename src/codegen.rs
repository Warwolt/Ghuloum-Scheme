use std::fmt::Write;

const BOOLEAN_TAG: i32 = 0b00011111;
const BOOLEAN_SHIFT: i32 = 7;

const CHARACTER_TAG: i32 = 0b00001111;
const CHARACTER_SHIFT: i32 = 8;

const FIXNUM_SHIFT: i32 = 2;

const EMPTY_LIST: i32 = 0b00101111;

#[allow(unused_must_use)]
pub fn emit_entry_point() -> String {
    let mut text = String::new();

    writeln!(&mut text, "    .text");
    writeln!(&mut text, "    .p2align 4,,15");
    writeln!(&mut text, ".global entry_point");
    writeln!(&mut text, "entry_point:");

    text
}

#[allow(unused_must_use)]
pub fn emit_boolean(val: bool) -> String {
    let mut text = String::new();

    // add type information to immediate
    let shifted_val = BOOLEAN_TAG | ((val as i32) << BOOLEAN_SHIFT);

    writeln!(&mut text, "    movl    ${}, %eax", shifted_val);
    writeln!(&mut text, "    ret");

    text
}

#[allow(unused_must_use)]
pub fn emit_character(val: i32) -> String {
    let mut text = String::new();

    // add type information to immediate
    let shifted_val = CHARACTER_TAG | ((val as i32) << CHARACTER_SHIFT);

    writeln!(&mut text, "    movl    ${}, %eax", shifted_val);
    writeln!(&mut text, "    ret");

    text
}

#[allow(unused_must_use)]
pub fn emit_fixint(val: i32) -> String {
    let mut text = String::new();

    // add type information to immediate
    let shifted_val = val << FIXNUM_SHIFT;

    writeln!(&mut text, "    movl    ${}, %eax", shifted_val);
    writeln!(&mut text, "    ret");

    text
}

#[allow(unused_must_use)]
pub fn emit_empty_list() -> String {
    let mut text = String::new();

    writeln!(&mut text, "    movl    ${}, %eax", EMPTY_LIST);
    writeln!(&mut text, "    ret");

    text
}
