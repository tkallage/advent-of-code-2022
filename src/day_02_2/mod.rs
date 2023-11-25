pub fn main(filepath: &str) -> i32 {
    let input_file_content: String = match std::fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(error) => panic!("Error while reading the file: {:?}", error),
    };

    let mut total_score = 0;

    enum Shape {
        Rock,
        Paper,
        Scissors,
    }
    enum Strategy {
        Win,
        Lose,
        Draw,
    }

    for line in input_file_content.lines() {
        if line.len() == 0 {
            continue;
        }
        let opponent_shape = match line.chars().nth(0) {
            Some('A') => Shape::Rock,
            Some('B') => Shape::Paper,
            Some('C') => Shape::Scissors,
            _ => panic!("Invalid shape"),
        };
        let strategy = match line.chars().nth(2) {
            Some('X') => Strategy::Lose,
            Some('Y') => Strategy::Draw,
            Some('Z') => Strategy::Win,
            _ => panic!("Invalid strategy"),
        };

        let my_shape = match (&strategy, &opponent_shape) {
            (Strategy::Draw, Shape::Rock)
            | (Strategy::Lose, Shape::Paper)
            | (Strategy::Win, Shape::Scissors) => Shape::Rock,
            (Strategy::Draw, Shape::Scissors)
            | (Strategy::Lose, Shape::Rock)
            | (Strategy::Win, Shape::Paper) => Shape::Scissors,
            (Strategy::Draw, Shape::Paper)
            | (Strategy::Lose, Shape::Scissors)
            | (Strategy::Win, Shape::Rock) => Shape::Paper,
        };

        const SCORE_LOSE: i32 = 0;
        const SCORE_DRAW: i32 = 3;
        const SCORE_WIN: i32 = 6;

        let score_from_game = match (&my_shape, &opponent_shape) {
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => SCORE_DRAW,
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => SCORE_WIN,
            (Shape::Scissors, Shape::Rock)
            | (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors) => SCORE_LOSE,
        };
        let score_from_shape = match my_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        total_score += score_from_game + score_from_shape;
    }

    println!("Total score: {}", total_score);

    total_score
}
