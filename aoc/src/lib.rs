use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod intcode;

#[macro_export]
macro_rules! get_row_input {
    ($d: expr) => {
        get_string_rows(format!("{}{}", "input_data/day_", $d).as_str());
    };
    ($d: expr, $t: ty) => {
        get_rows(format!("{}{}", "input_data/day_", $d).as_str());
    };
}

#[macro_export]
macro_rules! get_map_input {
    ($d: expr) => {
        get_map(format!("{}{}", "input_data/day_", $d).as_str());
    };
}

#[macro_export]
macro_rules! get_csv_input {
    ($d: expr, $e: expr, $t: ty) => {
        get_csv(format!("{}{}", "input_data/day_", $d).as_str(), $e);
    };
}

#[macro_export]
macro_rules! get_row_group {
    ($d: expr) => {
        get_string_row_groups(format!("{}{}", "input_data/day_", $d).as_str());
    };
}

pub fn get_string(file: &str) -> String {
    let input = File::open(file).unwrap();
    let mut buf_reader = BufReader::new(input);

    let mut result = String::new();
    buf_reader
        .read_line(&mut result)
        .expect("no string in file");
    result = result.replace('\n', "");
    return result;
}

pub fn get_csv<T: std::str::FromStr>(file: &str, delim: &str) -> Vec<T> {
    let input = File::open(file).unwrap();
    let mut buf_reader = BufReader::new(input);

    let mut result = String::new();
    buf_reader
        .read_line(&mut result)
        .expect("no string in file");
    result = result.replace('\n', "");
    let mut output = Vec::new();
    for val in result.split(delim) {
        match val.parse::<T>() {
            Ok(as_type) => output.push(as_type),
            Err(_) => panic!("Error parsing input"),
        };
    }

    return output;
}

pub fn get_rows<T: std::str::FromStr>(file: &str) -> Vec<T> {
    let mut output = Vec::new();
    let input = File::open(file).unwrap();
    let buf_reader = BufReader::new(input);

    for line in buf_reader.lines() {
        match line {
            Ok(string) => match string.parse::<T>() {
                Ok(as_type) => output.push(as_type),
                Err(_) => panic!("Error parsing input"),
            },
            Err(err) => panic!("Error reading input: {}", err),
        };
    }

    return output;
}

pub fn get_string_rows(file: &str) -> Vec<String> {
    let mut output = Vec::new();
    let input = File::open(file).unwrap();
    let buf_reader = BufReader::new(input);

    for line in buf_reader.lines() {
        match line {
            Ok(string) => {
                output.push(string);
            }
            Err(err) => panic!("Error reading input: {}", err),
        };
    }

    return output;
}

pub fn get_string_row_groups(file: &str) -> Vec<Vec<String>> {
    let mut output = vec![];
    let input = get_string_rows(file);
    let mut group = vec![];
    for r in input {
        if r.len() == 0 {
            output.push(group.clone());
            group.clear();
        } else {
            group.push(r);
        }
    }
    output.push(group);
    output
}

pub fn get_map(file: &str) -> HashMap<(usize, usize), char> {
    let rows = get_string_rows(file);
    let mut result = HashMap::new();
    for (y, r) in rows.iter().enumerate() {
        for (x, c) in r.chars().enumerate() {
            result.insert((x, y), c);
        }
    }
    result
}

pub fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}
