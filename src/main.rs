pub mod day1;

fn main() {
    let filepath = get_input::get_input("Please enter file path");
    if validate_file_path::validate(&filepath) {
        day1::expense_calc(&filepath);
        day1::expense_three_calc(&filepath)
    } else { println!("File doesn't exist") }

}

mod get_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}

mod validate_file_path {
    use std::path::Path;
    pub fn validate(filename: &str) -> bool {
        Path::new(filename).exists()
    }
}