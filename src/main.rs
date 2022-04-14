use std::{io, fmt::Result};
mod main_logic;

use main_logic::main_logic::main_logic_functions;
fn main() {
    let mut example = String::new();
    std::io::stdin().read_line(&mut example);
    //example = String::from("58*31+85*(10/10-3)/7*7");
    let result = main_logic_functions::main_func(&example);
    println!("{} = {}", &example, result);
}
