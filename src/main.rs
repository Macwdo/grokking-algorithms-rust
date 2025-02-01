use std::env;

mod chapter_1;
mod chapter_2;

fn main() {
    let args: Vec<String> = env::args().collect();

    // chapter_1::main(args);
    chapter_2::main(args);
}
