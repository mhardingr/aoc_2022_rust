use std::fs::File;
use std::path::Path;
use::std::io::prelude::*;
use std::collections::VecDeque;

pub struct Stack {
    pub crates: VecDeque<char>
}

impl Stack {
    pub fn push_front(mut self, s: char) -> () { 
        self.crates.push_front(s);
    }

    pub fn push_back(mut self, s: char) -> () { 
        self.crates.push_back(s);
    }

    pub fn peek_front(self) -> Result<char, &'static str> {
        if self.crates.len() == 0 {
            Err("Empty!")
        } else {
            Ok(self.crates[0])
        }
    }

    pub fn pop_front(mut self) -> Option<char> {
        self.crates.pop_front()
    }
}

fn parse_stacks(contents: &str) -> Vec<&Stack> {
    let mut stacks_v: Vec<&Stack> = Vec::new();

    // Read lines in reverse order, pushing into appropriate stacks
    for line in contents.lines(){
        if line.contains("1") {
            break;
        } else {
            assert_eq!(line.len(),35);
            line.chars() 
                .enumerate()
                .for_each(|(i, c)| {
                    if i % 4 == 1 {
                        println!("Char: {}", c);
                        stacks_v[i].push_back(c)
                    }
                });
        }
    }

    stacks_v
}

fn rearrange(contents: &str, mut stacks_v: Vec<&Stack>) {

}

fn init_contents_from_input() -> String {
    // Extract content from input file
    let mut file = File::open(Path::new("../input")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn run() {
    let contents: String = init_contents_from_input();

    // Initialize stacks from input's header
    let mut stacks_v: Vec<&Stack> = parse_stacks(&contents);
    
    // Rearrange stacks according to "move X ..." instructions in remaining lines
    rearrange(&contents, stacks_v);
    
    // Print tops of each stack
    let mut tops: String = String::new();
    for stack in stacks_v {
        tops.push(stack.peek_front().unwrap());
    }
    println!("Part 1: {}", tops);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
