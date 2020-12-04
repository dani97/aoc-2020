use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn expense_calc(filepath: &str) {
    let expenses = create_hash_map(filepath);
    for (expense, _flag) in expenses.iter() {
        match expenses.get(&(expense - 2020).abs()) {
            Some(&_num) => { println!("Expense of two {}", expense * (expense - 2020).abs()); break; },
            _ => {}
        }
    }
}

pub fn expense_three_calc(filepath: &str) {
    let expenses = create_vec(filepath);
    for mut index1 in 0..expenses.len()-2 {
        let mut index2 = index1 + 1;
        let mut index3 = expenses.len() - 1;
        while index2 < index3 {
            let triplet_sum = expenses[index1] + expenses[index2] + expenses[index3];
            if *&triplet_sum == 2020 {
                println!("Expenses of three {}", (expenses[index1] * expenses[index2] * expenses[index3]));
                break;
            } else if *&triplet_sum < 2020 {
                index2 = index2 + 1;
            } else {
                index3 = index3 - 1;
            }
        }
    }
}

fn create_hash_map(filepath: &str) -> HashMap<i32, bool> {
    let mut expenses = HashMap::new();
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                expenses.insert(ip.parse::<i32>().unwrap(), true);
            }
        }
    }
    expenses
}

fn create_vec(filepath: &str) -> Vec<i32> {
    let mut expenses = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                expenses.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    expenses.sort();
    expenses
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}