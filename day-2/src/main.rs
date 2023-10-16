// oponent has three choices
// enum OpenentChoice {

// }

struct ResultPoints {
    win: u8,
    draw: u8,
    lose: u8,
}

impl Default for ResultPoints {
    fn default() -> Self {
        ResultPoints {
            win: 6,
            draw: 3,
            lose: 0,
        }
    }
}

enum OpponentChoice {
    A, // rock
    B, // paper
    C, // scissors
}

impl OpponentChoice {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(OpponentChoice::A),
            'B' => Some(OpponentChoice::B),
            'C' => Some(OpponentChoice::C),
            _ => None,
        }
    }
}

enum YourChoice {
    X, // rock
    Y, // paper
    Z, // scissors
}

impl YourChoice {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'X' => Some(YourChoice::X),
            'Y' => Some(YourChoice::Y),
            'Z' => Some(YourChoice::Z),
            _ => None,
        }
    }
}

struct Round {
    your_choice: YourChoice,
    opponent_choice: OpponentChoice,
}

impl Round {
    fn play_round(&self) -> u8 {
        let result_points = ResultPoints::default();
        match &self.opponent_choice {
            OpponentChoice::A => match &self.your_choice {
                YourChoice::X => return result_points.draw + 1,
                YourChoice::Y => return result_points.win + 2,
                YourChoice::Z => return result_points.lose + 3,
            },
            OpponentChoice::B => match &self.your_choice {
                YourChoice::X => return result_points.lose + 1,
                YourChoice::Y => return result_points.draw + 2,
                YourChoice::Z => return result_points.win + 3,
            },
            OpponentChoice::C => match &self.your_choice {
                YourChoice::X => return result_points.win + 1,
                YourChoice::Y => return result_points.lose + 2,
                YourChoice::Z => return result_points.draw + 3,
            },
        };
    }
}

fn main() {
    let file = "test_input.txt";
    let file2 = "input.txt";
    let input = std::fs::read_to_string(&file2).expect(&format!("File '{}' does not exist", file));
    let mut score: u64 = 0;
    for round in input.lines() {
        let opponent_guess = round.chars().nth(0).unwrap();
        let your_guess = round.chars().last().unwrap();

        let current_round = Round {
            opponent_choice: OpponentChoice::from_char(opponent_guess)
                .expect("Expected Opponent Choice to be A B or C"),
            your_choice: YourChoice::from_char(your_guess)
                .expect("Expected Your Guess to be X, Y, or Z"),
        };

        let round_result = current_round.play_round();
        score += round_result as u64;
    }

    println!("{score}");
}
