use std::env;
use std::fs::read_to_string;
use regex::Regex;

fn parse_code(code: String) -> u32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for c in code.chars() {
        if c.is_numeric() {
            first.get_or_insert(c);
            last = Some(c);
        }
    }

    format!("{}{}", first.unwrap(), last.unwrap()).parse::<u32>().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Input File: {}", filename);

    let mut calibration_value = 0; 

    for line in read_to_string(filename).unwrap().lines() {
        let new_val = parse_code(line.to_string());
        calibration_value += parse_code(line.to_string());
    } 

    println!("Calibration Value: {}", calibration_value);
}
