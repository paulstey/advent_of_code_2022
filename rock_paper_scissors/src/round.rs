
#[derive(Debug)]
pub struct Round {
    elf: Shape,
    me: Shape,
    outcome: Outcome,
    score: i32,
}

impl Round {
    pub fn new(round_string: &str) -> Round {
        let char_vec = round_string
            .chars()
            .map(|c| c.to_string()) 
            .collect::<Vec<String>>();
       
        let elf_str = char_vec[0].clone();
        let me_str = char_vec[2].clone();

        let elf = Shape::from(&elf_str);
        let me = Shape::from(&me_str);

        let mut score = 0;

        let outcome = match elf {
            Shape::Rock => {
                if me == Shape::Paper {
                    score += 8;
                    Outcome::Win
                } else if me == Shape::Rock {
                    score += 4; 
                    Outcome::Tie
                } else {
                    score += 3;
                    Outcome::Lose
                }
            }
            
            Shape::Paper=> {
                if me == Shape::Scissors {
                    score += 9; 
                    Outcome::Win
                } else if me == Shape::Paper {
                    score += 5; 
                    Outcome::Tie
                } else {
                    score += 1;
                    Outcome::Lose
                }
            }

            Shape::Scissors => {
                if me == Shape::Rock{
                    score += 7; 
                    Outcome::Win
                } else if me == Shape::Scissors {
                    score += 6; 
                    Outcome::Tie
                } else {
                    score += 2;
                    Outcome::Lose
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

#[derive(Debug)]
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
