#[allow(dead_code)]
#[derive(Debug)]
pub struct Round {
    elf: Shape,
    me: Shape,
    outcome: Outcome,
    pub score: i32,
}

impl Round {
    pub fn new(round_string: &str) -> Round {
        let char_vec = round_string
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        let elf_str = char_vec[0].clone();
        let outcome_str = char_vec[2].clone();

        let elf = Shape::from(&elf_str);
        let outcome = match outcome_str.as_str() {
            "X" => Outcome::Lose,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => todo!(),
        };

        let mut score = 0;

        let me = match elf {
            Shape::Rock => {
                if outcome == Outcome::Win {
                    score += 8;
                    Shape::Scissors
                } else if outcome == Outcome::Tie {
                    score += 4;
                    Shape::Rock
                } else {
                    score += 3;
                    Shape::Paper
                }
            }

            Shape::Paper => {
                if outcome == Outcome::Win {
                    score += 9;
                    Shape::Scissors
                } else if outcome == Outcome::Tie {
                    score += 5;
                    Shape::Paper
                } else {
                    score += 1;
                    Shape::Rock
                }
            }

            Shape::Scissors => {
                if outcome == Outcome::Win {
                    score += 7;
                    Shape::Rock
                } else if outcome == Outcome::Tie {
                    score += 6;
                    Shape::Scissors
                } else {
                    score += 2;
                    Shape::Paper
                }
            }
        };

        Round {
            elf,
            me,
            outcome,
            score,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Tie,
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from(letter: &str) -> Self {
        let rock = ["A", "X"];
        let paper = ["B", "Y"];

        if rock.contains(&letter) {
            Shape::Rock
        } else if paper.contains(&letter) {
            Shape::Paper
        } else {
            Shape::Scissors
        }
    }
}
