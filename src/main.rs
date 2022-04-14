use std::{io, fmt::Result};
mod main_logic;

use main_logic::main_logic::main_logic_functions;
fn main() {
    let mut example = String::new();
    std::io::stdin().read_line(&mut example);
    //example = String::from("5()");
    let result = main_logic_functions::main_func(&example);
    println!("{} = {}", &example, result);
}
