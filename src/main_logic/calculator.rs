pub mod calculator {
    pub mod calculator_functions{
        fn check_division_by_zero(value: f64) -> f64{
            match value == 0 as f64{
                true => panic!("Division on zero is not possible!"),
                _ => return value,
            } 
        }

        pub fn calc_with_operation(first_value_string: &f64, second_value_string: &f64, operation: &String) -> f64{
            match operation.as_str(){
                "+" => return first_value_string + second_value_string,
                "-" => return first_value_string -second_value_string,
                "*" => return first_value_string * second_value_string,
                "/" => return first_value_string / check_division_by_zero(second_value_string.clone()),
                _ => panic!("Value must be digital, without other symbols {}.", operation),
            }
        }
    }
}
