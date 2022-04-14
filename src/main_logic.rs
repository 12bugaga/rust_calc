mod calculator;
mod enum_value;

use calculator::calculator::calculator_functions::{*};
use enum_value::{*};

pub mod main_logic{
    pub mod main_logic_functions{
        use core::panic;

        use crate::main_logic::enum_value::{OPERATION_TYPE, OPERATION_PRIORITY};

        pub fn main_func(example: &String) -> String{
            let char_example: Vec<char> = example.trim().chars().collect();
            let actions_vec: &mut Vec<char> =&mut Vec::new();
            let value_vec: &mut Vec<f64> = &mut Vec::new();
            let mut templar_value_f64: f64;
            let mut templar_value = String::from("");

            let mut i = 0;
            let mut c: char;
            while i < char_example.len(){
                c = char_example[i];

                if c.is_digit(10){
                    templar_value.push(c);
                }
                else if OPERATION_TYPE.contains(&c){
                    match templar_value.is_empty() {
                        true => (),
                        false => {
                            value_vec.push(parse_to_f64(templar_value));
                            templar_value = String::from("");
                        },
                    }
                    
                    if actions_vec.len() == 0{
                        actions_vec.push(c);
                    }
                    else if OPERATION_PRIORITY[&c] != 0 &&
                    OPERATION_PRIORITY[&c] > OPERATION_PRIORITY[actions_vec.get(actions_vec.len() -1 ).unwrap()]{
                        actions_vec.push(c);
                    }
                    else if OPERATION_PRIORITY[&c] != 0 && 
                    OPERATION_PRIORITY[&c] <= OPERATION_PRIORITY[&actions_vec.get(actions_vec.len() -1 ).unwrap()]{
                            
                        templar_value_f64 = calc_operation(value_vec, actions_vec);
                        value_vec.push(templar_value_f64);
                        i -= 1;
                    }
                    else if c == '(' {
                        actions_vec.push(c);
                    }
                    else if c == ')'{
                        while actions_vec.get(actions_vec.len() -1 ).unwrap().to_string() != "("{
                            
                        templar_value_f64 = calc_operation(value_vec, actions_vec);
                        value_vec.push(templar_value_f64);
                        }
                        actions_vec.pop();
                    }
                }
                
                else{
                    panic!("Please check your example, mb it contain other symbols! => {}!", c.to_string());
                }
                i += 1;
            }

            value_vec.push(parse_to_f64(templar_value));
            while value_vec.len() != 1{
                templar_value_f64 = calc_operation(value_vec, actions_vec);
                value_vec.push(templar_value_f64);
            }

            return match value_vec.pop(){
                Some(top) => top.to_string(),
                None => panic!("Not enough sumbols!"),
            };
        }

        fn parse_to_f64(value: String) -> f64{
            match value.trim().parse::<f64>() {
                Ok(num) => num,
                Err(_) => panic!("Value must be digital, without other symbols {}.", value)
            }
        }

        fn calc_operation(value_vec: &mut Vec<f64>, actions_vec: &mut Vec<char>) -> f64{
            let operation = match actions_vec.pop(){
                Some (o) => o,
                None => panic!("Please check your example"),
            };
            let last_value = match value_vec.pop(){
                Some(top) => top,
                None => panic!("Not enough sumbols!"),
            };
            let prev_last_value = match value_vec.pop(){
                Some(top) => top,
                None => panic!("Not enough sumbols!"),
            };
            return crate::main_logic::calculator::calculator::calculator_functions::
                            calc_with_operation(&prev_last_value, 
                                            &last_value, 
                                            &operation.to_string()
                                            );
        }
    }
}