use std::fs::File;
use std::path::Path;
use::std::io::prelude::*;
use std::collections::VecDeque;

fn parse_stacks(contents: &str) -> Vec<VecDeque<char>> {
    let mut stacks_v: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    // Read lines in order, but stacks will be assembled in reverse
    // which means we need to "enqueue"/push_front each crate
    for line in contents.lines(){
        if line.contains("1") {
            break;
        } else {
            assert_eq!(line.len(),35);
            line.chars() 
                .enumerate()
                .for_each(|(i, c)| {
                    if i % 4 == 1 && c != ' ' {
                        let stack_i = i as usize / 4;
                        stacks_v[stack_i].push_front(c)
                    }
                });
        }
    }

    stacks_v
}

fn rearrange_p1(contents: &str, stacks_v: &mut Vec<VecDeque<char>>) {
    // Read move lines (starting at 11th line)
    contents.lines()
        .skip(10)
        .for_each(|line| {
            let moves: Vec<usize> = line.split(' ')
                .filter(|&s| if let Ok(_) = s.parse::<usize>() {true} else {false})
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            assert_eq!(moves.len(), 3);
            let (pop_count, from_i, to_j) = (moves[0], moves[1]-1, moves[2]-1);
            assert!(from_i < stacks_v.len() && to_j < stacks_v.len());
            for _ in 0..pop_count {
                // Note: head of stack is "back", so pop_back and push_back are used
                let c: char = stacks_v[from_i].pop_back().unwrap();
                stacks_v[to_j].push_back(c);
            }
        });
}

fn rearrange_p2(contents: &str, stacks_v: &mut Vec<VecDeque<char>>) {
    // Read move lines (starting at 11th line)
    contents.lines()
        .skip(10)
        .for_each(|line| {
            let moves: Vec<usize> = line.split(' ')
                .filter(|&s| if let Ok(_) = s.parse::<usize>() {true} else {false})
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            assert_eq!(moves.len(), 3);
            let (push_count, from_i, to_j) = (moves[0], moves[1]-1, moves[2]-1);
            assert!(from_i < stacks_v.len() && to_j < stacks_v.len());
            // Use an intermediate stack to "enqueue" push_count crates onto to_stack
            let mut int_stack: VecDeque<char> = VecDeque::new();
            for _ in 0..push_count{
                // Note: head of stack is "back", so pop_back and push_back are used
                let c: char = stacks_v[from_i].pop_back().unwrap();
                int_stack.push_back(c);
            }
            for _ in 0..push_count {
                stacks_v[to_j].push_back(int_stack.pop_back().unwrap());
            }
        });
}

fn init_contents_from_input() -> String {
    // Extract content from input file
    let mut file = File::open(Path::new("input")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

pub fn run() {
    let contents: String = init_contents_from_input();

    // Initialize stacks from input's header
    let mut stacks_v_p1: Vec<VecDeque<char>> = parse_stacks(&contents);
    let mut stacks_v_p2: Vec<VecDeque<char>> = parse_stacks(&contents);
    
    // Rearrange stacks according to "move X ..." instructions in remaining lines
    rearrange_p1(&contents, &mut stacks_v_p1);
    rearrange_p2(&contents, &mut stacks_v_p2);
    
    // Print tops of each stack
    let mut tops_p1: String = String::new();
    let mut tops_p2: String = String::new();
    // Top of stack is last element in VecDeque
    stacks_v_p1.iter().for_each(|s| tops_p1.push(*(s.get(s.len()-1).unwrap())));
    println!("Part 1: {}", tops_p1);
    stacks_v_p2.iter().for_each(|s| tops_p2.push(*(s.get(s.len()-1).unwrap())));
    println!("Part 2: {}", tops_p2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
