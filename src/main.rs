
mod parser;

use crate::parser::ParseRules;

fn main() {

    let _ = parser::Parser::new("".to_string());

    println!("Hello, world!");
}
