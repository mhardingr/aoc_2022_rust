use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Read in input into vector 'lines'
    let inp_path = Path::new("input");
    let mut file = File::open(&inp_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();

    // Collect total calorie-sums into vector, one entry per elf
    let mut cal_sums:  Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for line in lines {
        if line == "" { 
            cal_sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    // Sort cal_sums inplace
    cal_sums.sort();
    let biggest_sum = cal_sums[cal_sums.len()-1];
    let biggest_3_sum : i32 = cal_sums[cal_sums.len()-3..].iter().sum();

    println!("Biggest {}", biggest_sum);
    println!("Sum biggest 3 {}", biggest_3_sum); 
        
}
