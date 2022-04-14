use phf::phf_map;

pub static OPERATION_PRIORITY: phf::Map<char, u32> = phf_map! {
    '(' => 0,
    ')' => 0,
    '+' => 1,
    '-' => 2,
    '*' => 3,
    '/' => 4,
};

pub const OPERATION_TYPE: [char; 6] = ['+', '-', '/', '*', '(', ')'];