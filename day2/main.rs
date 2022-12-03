use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn given_outcome_get_shape(opp_shape: &str, given_outcome: &str) -> String {
    // Determine shape
    let my_shape: &str = match [opp_shape, given_outcome] {
        ["A", "X"] => "S",
        ["A", "Y"] => "R",
        ["A", "Z"] => "P",
        ["B", "X"] => "R",
        ["B", "Y"] => "P",
        ["B", "Z"] => "S",
        ["C", "X"] => "P",
        ["C", "Y"] => "S",
        ["C", "Z"] => "R",
        _ => panic!("Bad input! {} with outcome {}", opp_shape, given_outcome),
    };
    
    return my_shape.to_string();
}

fn given_shapes_get_outcome(opp_shape: &str, given_shape: &str) -> String {
    // Determine outcome
    let outcome: &str = match [opp_shape, given_shape] {
        ["A", "X"] => "D",
        ["A", "Y"] => "W",
        ["A", "Z"] => "L",
        ["B", "X"] => "L",
        ["B", "Y"] => "D",
        ["B", "Z"] => "W",
        ["C", "X"] => "W",
        ["C", "Y"] => "L",
        ["C", "Z"] => "D",
        _ => panic!("Bad input! {} vs. {}", opp_shape, given_shape),
    };

    return outcome.to_string();
}

fn main() {
    // Read in input into vector 'lines'
    let inp_path = Path::new("input");
    let mut file = File::open(&inp_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();

    let shapes_scores_m = HashMap::from([
        ("A", 1), // Opponent shapes
        ("B", 2),
        ("C", 3),
        ("X", 1), // When XYZ -> shapes
        ("Y", 2),
        ("Z", 3),
        ("R", 1), // Internally used shapes
        ("P", 2),
        ("S", 3),
    ]);

    let outcome_scores_m = HashMap::from([
        ("L", 0),
        ("D", 3),
        ("W", 6),
        ("X", 0), // Part 2 - Loss
        ("Y", 3), // ... Draw
        ("Z", 6), // ... Win

    ]);

    // Iterate over lines to determine scores ("given shapes" and "given outcomes")
    let mut gs_tot_score: i32 = 0;
    let mut go_tot_score: i32 = 0;
    for line in lines {
        let instr: Vec<&str> = line.split(" ").collect();
        let opp_shape: &str = instr[0];
        let (given_shape, given_outcome): (&str, &str) = (instr[1], instr[1]);

        // Determine outcome (part 1)
        let my_outcome = given_shapes_get_outcome(opp_shape, given_shape);
        // Determine shape (part 2)
        let my_shape = given_outcome_get_shape(opp_shape, given_outcome);

        // Part 1 score
        let gs_score: i32 = shapes_scores_m.get(given_shape).unwrap() +
            outcome_scores_m.get(&*my_outcome).unwrap();
        gs_tot_score += gs_score;

        // Part 2 score
        let go_score: i32 = shapes_scores_m.get(&*my_shape).unwrap() +
            outcome_scores_m.get(given_outcome).unwrap();
        go_tot_score += go_score;
    }
    println!("Given shapes (part 1), total score: {}", gs_tot_score);
    println!("Given outcomes (part 2), total score: {}", go_tot_score);
}
