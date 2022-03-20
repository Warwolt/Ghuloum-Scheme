use std::fs;

fn main() {
    #[rustfmt::skip]
    let data =
"    .text
    .p2align 4,,15
.global entry_point

entry_point:
    movl    $42, %eax
    ret
";
    fs::create_dir_all("./build").expect("Unable to create build directory");
    fs::write("./build/program.s", data).expect("Unable to write file");
}
