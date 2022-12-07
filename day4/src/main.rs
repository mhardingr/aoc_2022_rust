use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn parse_min_max(id_str: &str) -> (u32, u32) {
    let split: Vec<&str> = id_str.trim().split("-").collect();
    let min: u32 = split[0].parse().unwrap();
    let max: u32 = split[1].parse().unwrap();

    (min, max)
}

fn parse_pair(pair_line: &str) -> Vec<&str> {
    pair_line.trim().split(",").collect::<Vec<&str>>()
}

fn main() {
    // Read in input into Vec<&str> 'lines'
    let inp_path = Path::new("../input");
    let mut file = File::open(&inp_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();

    let mut count_redundant_pairs = 0; // Part 1 count
    let mut count_overlapping_pairs = 0; // Part 2 count
    for line in lines {
        let pair: Vec<&str> = parse_pair(line);
        let (elf1_min, elf1_max): (u32, u32) = parse_min_max(pair[0]);
        let (elf2_min, elf2_max): (u32, u32) = parse_min_max(pair[1]);

        if elf1_min < elf2_min {
            // First elf might contain second elf's ids
            if elf1_max >= elf2_max  {
                count_redundant_pairs +=1;
            }
            if elf1_max >= elf2_min {
                count_overlapping_pairs +=1;
            }
        } else if elf1_min > elf2_min {
            // Second elf might contain first elf's ids
            if elf1_max <= elf2_max {
                count_redundant_pairs +=1;
            }
            if elf2_max >= elf1_min {
                count_overlapping_pairs +=1;
            }
        } else {
            // Shared min means overlap unavoidable
            count_redundant_pairs +=1;
            count_overlapping_pairs +=1;
        }
    }
    println!("(Part 1) redundant pairs counted: {}", count_redundant_pairs);
    println!("(Part 2) redundant pairs counted: {}", count_overlapping_pairs);
}
