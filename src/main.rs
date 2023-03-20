use std::{env, fs::File};
use std::io::{BufReader, BufRead, Lines};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Provide an input file.");
        return;
    }

    if true {

    } else {

    }
    
    let file_path: &str = &args[1];
    let max_line_len = get_lines(file_path)
                        .map(|x| x.unwrap().len())
                        .max()
                        .unwrap_or(0);

    if max_line_len == 0 {
        println!("File is empty?");
        std::process::exit(0);
    }

    let mut line_iter = get_lines(file_path).map(|x| x.unwrap()).into_iter();

    let mut prev_line: String = match line_iter.next() {
        Some(l) => format_line_end(l, max_line_len),
        None => return
    };

    while let Some(mut line) = line_iter.next() {
        line = format_line_end(line, max_line_len);

        if let Some('}') = line.trim_start().chars().nth(0) {
            line = line.replacen("}", "", 1);
            prev_line.push('}');
        }

        println!("{}", prev_line);
        prev_line = line;
    }

    println!("{}", prev_line);
}

static CHARS_TO_FORMAT: &str = ";}{,";

fn format_line_end(line: String, max_len: usize) -> String {
    let mut line_vec: Vec<u8> = line.into();

    let line_len: usize = line_vec.len();
    let ws_amout: usize = if line_len == max_len { 0 } else { max_len - line_len };

    let mut stop_index = 0;
    for (i, b) in line_vec.iter_mut().rev().enumerate() {
        let c: char = *b as char;

        if !CHARS_TO_FORMAT.contains(c) && !char::is_whitespace(c) {
            stop_index = line_len - i;
            break;
        }
    }

    let ws: Vec<u8> = " ".repeat(ws_amout).into();
    line_vec.splice(stop_index..stop_index, ws);

    let formated_line: String = match String::from_utf8(line_vec) {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    formated_line
}

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    match File::open(path) {
        Ok(file) => BufReader::new(file).lines(),
        Err(error) => panic!("Error opening '{}': {}", path, error)
    }
}

