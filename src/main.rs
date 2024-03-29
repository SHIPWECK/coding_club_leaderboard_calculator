use std::{collections::HashMap, fs::read_to_string};

const COLUMNS_SKIPPED: usize = 5;

fn main() {
    let mut scores: HashMap<&str, f64> = HashMap::new();
    let csv_string =
        read_to_string("Coding Club Scores - Sheet1.csv").expect("Could not find CSV file");

    for (line_num, line) in csv_string.trim().lines().skip(1).enumerate() {
        let mut line_iter = line.split(',');
        let name = line_iter
            .next()
            .unwrap_or_else(|| panic!("Error reading name for line {line_num}"));

        let score = line_iter
            .nth(COLUMNS_SKIPPED)
            .unwrap_or_else(|| panic!("Error getting score for {name} on line {line_num}"))
            .parse::<f64>()
            .unwrap_or_else(|_| panic!("Error parsing score for {name} on line {line_num}"));

        *scores.entry(name).or_default() += score;
    }

    let mut leader_board = scores.into_iter().collect::<Vec<_>>();

    // sorts the leaderboard in descending order
    leader_board.sort_by(|(name1, score1), (name2, score2)| {
        score1
            .partial_cmp(score2)
            .unwrap_or_else(|| {
                panic!(
                    "Error comparing values for {name1} and {name2} (values: {score1}, {score2})"
                )
            })
            .reverse()
    });

    for (idx, (name, score)) in leader_board.into_iter().enumerate() {
        println!("{}. {name}: {score} points", idx + 1);
    }
}
