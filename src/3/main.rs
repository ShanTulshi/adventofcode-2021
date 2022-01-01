use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::vec::Vec;

fn meets_oxygen_criteria(counts: &Vec<(u64, u64)>, line: String, position: usize) -> bool {
    let c = counts[position];
    if c.0 > c.1 {
        return line.as_bytes()[position] == '0' as u8;
    } else {
        return line.as_bytes()[position] == '1' as u8;
    }
}

fn meets_co2_criteria(counts: &Vec<(u64, u64)>, line: String, position: usize) -> bool {
    let c = counts[position];
    if c.1 < c.0 {
        return line.as_bytes()[position] == '1' as u8;
    } else {
        return line.as_bytes()[position] == '0' as u8;
    }
}

fn main() {
    let file = File::open(Path::new("./src/3/sample_input.txt")).unwrap();
    let mut common_digits: Vec<(u64, u64)> = vec![];
    let lines: Vec<String> = BufReader::new(file).lines().map(|maybe_line| maybe_line.unwrap()).collect();

    for line in &lines {
        if common_digits.len() < line.len() {
            common_digits.extend(line.chars().map(|elem| if elem == '0' { return (1, 0) } else { return (0, 1) } ));
        } else {
            for (i, c) in line.chars().enumerate() {
                if c == '0' {
                    common_digits[i].0 += 1;
                } else if c == '1' {
                    common_digits[i].1 += 1;
                } else {
                    println!("invalid input encountered: {}", c);
                    return;
                }
            }
        }
    };

    let mut gamma_rate = 0u64;
    let mut epsilon_rate = 0u64;
    for (zeros, ones) in common_digits {
        if zeros > ones {
            gamma_rate <<= 1;
            epsilon_rate = (epsilon_rate << 1) + 1;
        } else {
            gamma_rate = (gamma_rate << 1) + 1;
            epsilon_rate <<= 1;
        }
    }

    println!("the gamma_rate in part one is: {}", gamma_rate);
    println!("the epsilon_rate in part one is: {}", epsilon_rate);
    println!("the answer to part one is: {}", gamma_rate * epsilon_rate);

    // Part 2
    let mut o2_rating = lines.enumerate();
    let mut pos = 0;
    while o2_rating.len() > 1 {
        for (i, elem) in o2_rating {
            if !meets_oxygen_criteria(&common_digits, elem.to_string(), pos) {
                o2_rating.remove(i);
            }
        }
        pos +=1;
    }
    let mut o2 = 0u64;
    for c in o2_rating[0].chars() {
        if c == '0' {
            o2 <<= 1;
        } else if c == '1' {
            o2 = (o2 << 1) + 1
        }
    }

    println!("the o2 rating is {}", o2);
}