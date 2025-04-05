/// adds continuous numbering to '######' headers of md file
///
/// 
/// Usage
/// 
/// in the md_numbering package
/// cargo run -- md_file_name
/// 
/// in rust_sandbox dir
/// cargo run -p md_numbering -- test_file.md
/// 
/// file must be in the same dir when cargo run.
/// all headers '######' will be continuos numbered
/// 
/// 
/// header formats:
/// ###### 4. What are the methods of memory management?
/// ######  49. What are the methods of memory management?
/// ###### 4999. What are the methods of memory management?
/// ######  4      What are the methods of memory management?
/// ###### 49   What are the methods of memory management?
/// 
/// 
/// Example:
/// cargo run -- test_file.md

use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::env;
use regex::Regex;

const QUESTIONS_HEADER: &str = "######";
const RE_HEADER: &str = r"^###### ";

const OUT_FILE_PREFIX: &str = "out_";
const RE_QUESTION: &str = r"###### +(\d{0,4}\.? +)?(?<question>.+)";

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    // let arg = args[1];
    let line_count =  count_line_in_file_by_reg(&args[1]).unwrap();
    println!("line_count {}", line_count);

}

fn count_line_in_file_by_reg(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
// fn count_line_in_file_by_reg(path: &str, re_str: &str) -> usize {
    let in_file = File::open(path)?;
    let reader = BufReader::new(in_file);
    // println!("file exist");

    let mut out_file = OpenOptions::new()
        .write(true)
        .create(true)
        // .append(true)
        .open(format!("{}{}", OUT_FILE_PREFIX, path))?;


    let mut counter = 0;

    let line_re = Regex::new(RE_HEADER).unwrap();
    let question_re = Regex::new(RE_QUESTION).unwrap();

    for line in reader.lines() {
        match line {
            // line is a String
            
            Ok(line) => if line_re.is_match(&line) {
                counter += 1;
                // println!("line match");
                let Some(caps) = question_re.captures(&line) else {
                    println!("{}", &line);
                    continue;
                };
                out_file.write_all(format!("{} {}. {}\n",QUESTIONS_HEADER, counter, &caps["question"]).as_bytes())?;
            }
            else {
                out_file.write_all(format!("{}\n", line).as_bytes())?;

                // println!("line not match");
            },
            Err(err) => panic!("{}", err),
            // Ok(line) => process_line(line),
            // Err(err) => handle_error(err),
        }
    }
    Ok(counter)
    
}
