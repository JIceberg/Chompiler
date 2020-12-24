mod parser;

use std::{fs::File, io::Write};
use std::path::Path;

fn main() {
    let v = parser::lexer::lex("int main() { return 2; }");
    println!("{:?}", v);
    let prog = parser::parse::Parser::new(v).parse();
    println!("{:?}", prog);
    let asm = parser::generate::Generator::new().generate(prog);
    let mut path = Path::new("src/output.s");
    let mut file = File::create(path).expect("Failed");
    file.write_all(asm.as_bytes().as_ref());
}