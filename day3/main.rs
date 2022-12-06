use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

fn get_priority(item: char) -> u32 {
    // We know chars are 4B in Rust, and we can assume we're using ASCII (1B, unsigned)
    // characters as items in the sack
    let priority: u32 = match item{
        'a'..='z' => item as u32 - 97 + 1,
        'A'..='Z' => item as u32 - 65 + 27,
        _ => panic!("Invalid char found! '{}'", item)
    };

    return priority;
}

fn parse_rucksets_from_line(line: &str, first_set: &mut HashSet<char>, second_set: &mut HashSet<char>) {
    assert_eq!(line.len() % 2, 0);

    // Extract rucksack substrings
    // Note: assumes that line only contains single-codepoint chars
    let first_rucksack: Vec<char> = line[..line.len()/2].chars().collect();
    let second_rucksack: Vec<char> = line[line.len()/2 ..].chars().collect();

    // Extend caller's rucksets with contents of rucksacks
    first_set.extend(first_rucksack.iter());
    second_set.extend(second_rucksack.iter());
}

fn parse_rucksets_from_group_of_lines(lines: &Vec<&str>) -> (HashSet<char>, HashSet<char>, HashSet<char>) {
    // Assumes lines is composed of 3 lines 
    assert_eq!(lines.len(), 3);

    return (HashSet::<char>::from_iter(lines[0].chars()),
            HashSet::<char>::from_iter(lines[1].chars()),
            HashSet::<char>::from_iter(lines[2].chars()));
}

fn main() {
    // Read in input into vector 'lines'
    let inp_path = Path::new("input");
    let mut file = File::open(&inp_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();

    /* Part 1 */
    // Each line represents 2 rucksacks-compartments of equal length (of characters)
    let mut tot_priorities_p1: u32 = 0;
    for line in lines.clone().collect::<Vec<&str>>() {
        // We want the contents of each rucksack to be organized into sets of
        // chars
        let mut first_ruckset: HashSet<char> = HashSet::new();
        let mut second_ruckset: HashSet<char> = HashSet::new();
        parse_rucksets_from_line(line, &mut first_ruckset, &mut second_ruckset);
        
        // Next, find the (assumed singular) 'intersecting' item between both
        // rucksack-components
        let mut intersection = first_ruckset.intersection(&second_ruckset);
        // assert_eq!(intersection.size_hint()[0], 1);

        // Add priority of intersecting item to tot_priorities
        tot_priorities_p1 += get_priority(*intersection.next().unwrap());
    }

    /* Part 2 */
    // Each line represents a rucksack, and every 3 lines represents a group
    // We want to find the common item across each successive group of 3 sacks
    // and return sum of priorities of group-common items across all groups
    let mut tot_priorities_p2 = 0;
    for group in lines.clone().collect::<Vec<&str>>().chunks(3) {
        let sacks_v = Vec::from(group);
        // Extract sets of items found in each rucksack
        let (first_ruckset, second_ruckset, third_ruckset) = parse_rucksets_from_group_of_lines(&sacks_v);

        // Run intersection across 3 rucksets (first&second and first&third)
        let intersect_12 = first_ruckset.intersection(&second_ruckset);
        let intersect_13 = first_ruckset.intersection(&third_ruckset);

        // And intersect the intersections above to get common item across all 3
        // TODO
        let hs_int_12: HashSet<char> = HashSet::<char>::from_iter(intersect_12.collect::<String>().chars());
        let hs_int_13: HashSet<char> = HashSet::<char>::from_iter(intersect_13.collect::<String>().chars());
        let mut intersection = hs_int_12.intersection(&hs_int_13);

        // Add priority of group-common item to running sum
        tot_priorities_p2 += get_priority(*intersection.next().unwrap());
    }
    
    println!("Part 1 tot_priorities: {}", tot_priorities_p1);
    println!("Part 2 tot_priorities: {}", tot_priorities_p2);
}
