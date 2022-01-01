use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::option::Option::{Some, None};

fn part2(file: File) {
    let mut count = 0;
    let mut prev_window: Vec<u64> = Vec::new();
    let mut cur_window: Vec<u64> = Vec::new();
    for line in BufReader::new(file).lines() {
        let parsed_num = line.unwrap().parse::<u64>().unwrap();
        if prev_window.len() == 0 {
            prev_window.push(parsed_num);
        } else if(cur_window.len() < 3) {
            cur_window.push(parsed_num);
        }
        prev_window.push(*cur_window.first().unwrap())
        
    }
}

fn main() {
    if let Ok(file) =  File::open(Path::new("./src/1/input.txt")) {
        // Part 1
        let mut count = 0;
        let mut prev_num: std::option::Option<u64> = None;
        let lines = BufReader::new(file).lines();
        for line in lines {
            if let Ok(num) = line {
                let parsed_num = num.parse::<u64>().unwrap();
                if let Some(prev) = prev_num {
                    if prev < parsed_num {
                        count += 1;
                    }
                }
                prev_num = Some(parsed_num);
            } else {
                println!("There was a problem parsing a line: {:?}", line);
            }
        }
        println!("The answer is {}!", count);
    } else {
        println!("There was an error opening the file");
    }
}
