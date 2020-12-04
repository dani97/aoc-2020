use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::ops::Index;

pub fn get_num_correct_password(filename: &str) -> i32 {
    let captures: Vec<(i32, i32, char, String)> = parse_input_file(filename);
    let mut valid_passwords = 0;
    for capture in captures {
        if check_password(capture.0, capture.1, capture.2, &capture.3) {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

pub fn get_num_correct_password_toboggan(filename: &str) -> i32 {
    let captures: Vec<(i32, i32, char, String)> = parse_input_file(filename);
    let mut valid_passwords = 0;
    for capture in captures {
        if check_password_toboggan(capture.0, capture.1, capture.2, &capture.3) {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn check_password_toboggan(index1: i32, index2: i32, rule_char: char, password: &str) -> bool {
    (password.as_bytes()[(index1 -1) as usize] == rule_char as u8) ^ (password.as_bytes()[(index2 -1) as usize ] == rule_char as u8)
}

fn check_password(lower_limit: i32, upper_limit: i32, rule_char: char, password: &str) -> bool {
    let count = password.matches(rule_char).count() as i32;
    count >= lower_limit && count <= upper_limit
}

fn parse_input_file(filename: &str) -> Vec<(i32, i32, char, String)> {
    let contents = fs::read_to_string(filename).expect("File read failed");
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)\s?").unwrap();
    }
    let mut captured_passwords = Vec::new();
    for cap in RE.captures_iter(&contents) {
        let rule = (cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap(), cap[3].parse::<char>().unwrap(), cap[4].parse::<String>().unwrap());
        captured_passwords.push(rule);
    }
    captured_passwords
}