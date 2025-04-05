/// Count all line that match regex expression
/// 
/// Usage
/// 
/// cargo run -- file_name regex
/// 
/// file must be in the same dir when cargo run
/// regex - regex expression
/// 
/// Example:
/// cargo run -- test_file.md "^###### "
/// count all lane whit header ######

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    // let arg = args[1];
    let line_count =  count_line_in_file_by_reg(&args[1], &args[2]).unwrap();
    println!("line_count {}", line_count);

}

fn count_line_in_file_by_reg(path: &str, re_str: &str) -> Result<usize, Box<dyn std::error::Error>> {
// fn count_line_in_file_by_reg(path: &str, re_str: &str) -> usize {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    println!("file exist");

    let mut counter = 0;

    let re = Regex::new(re_str).unwrap();

    for line in reader.lines() {
        match line {
            // line is a String
            
            Ok(line) => if re.is_match(&line) {counter += 1;},
            Err(err) => panic!("{}", err),
            // Ok(line) => process_line(line),
            // Err(err) => handle_error(err),
        }
    }
    Ok(counter)
    
}
