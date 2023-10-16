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
    fn from_char(c: &char) -> Option<Self> {
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
    fn from_char(c: &char) -> Option<Self> {
        match c {
            'X' => Some(YourChoice::X),
            'Y' => Some(YourChoice::Y),
            'Z' => Some(YourChoice::Z),
            _ => None,
        }
    }
}

struct ChoicePoints {
    x: u8,
    y: u8,
    z: u8,
}

impl Default for ChoicePoints {
    fn default() -> Self {
        ChoicePoints { x: 1, y: 2, z: 3 }
    }
}

struct Round {
    your_choice: YourChoice,
    opponent_choice: OpponentChoice,
    round_num: u64,
}

impl Round {
    fn desired_outcome(round_num: u64) -> &'static str {
        match round_num {
            1 => "draw",
            2 => "lose",
            3 => "win",
            _ => panic!("Invalid round number"),
        }
    }

    fn optimal_choice(outcome: &str, opponent_choice: &OpponentChoice) -> YourChoice {
        match outcome {
            "draw" => match opponent_choice {
                OpponentChoice::A => YourChoice::X,
                OpponentChoice::B => YourChoice::Y,
                OpponentChoice::C => YourChoice::Z,
            },
            "lose" => match opponent_choice {
                OpponentChoice::A => YourChoice::Z,
                OpponentChoice::B => YourChoice::X,
                OpponentChoice::C => YourChoice::Y,
            },
            "win" => match opponent_choice {
                OpponentChoice::A => YourChoice::Y,
                OpponentChoice::B => YourChoice::Z,
                OpponentChoice::C => YourChoice::X,
            },
            _ => panic!("Invalid outcome"),
        }
    }

    fn play_round(&self) -> u8 {
        let result_points = ResultPoints::default();
        let choice_points = ChoicePoints::default();
        match &self.opponent_choice {
            OpponentChoice::A => match &self.your_choice {
                YourChoice::X => return result_points.draw + choice_points.x,
                YourChoice::Y => return result_points.win + choice_points.y,
                YourChoice::Z => return result_points.lose + choice_points.z,
            },
            OpponentChoice::B => match &self.your_choice {
                YourChoice::X => return result_points.lose + choice_points.x,
                YourChoice::Y => return result_points.draw + choice_points.y,
                YourChoice::Z => return result_points.win + choice_points.z,
            },
            OpponentChoice::C => match &self.your_choice {
                YourChoice::X => return result_points.win + choice_points.x,
                YourChoice::Y => return result_points.lose + choice_points.y,
                YourChoice::Z => return result_points.draw + choice_points.z,
            },
        }
    }
}

fn main() {
    let file = "test_input.txt";
    let file2 = "input.txt";
    let input = std::fs::read_to_string(&file2).expect(&format!("File '{}' does not exist", file));
    let mut score: u64 = 0;
    let mut round_num: u64 = 1;

    for round in input.lines() {
        let opponent_guess_char = round.chars().nth(0).unwrap();
        let opponent_guess = OpponentChoice::from_char(&opponent_guess_char)
            .expect("Expected Opponent Choice to be A B or C");

        let desired = Round::desired_outcome(round_num);
        let your_choice = Round::optimal_choice(desired, &opponent_guess);

        let current_round = Round {
            opponent_choice: opponent_guess,
            your_choice,
            round_num,
        };

        if round_num < 3 {
            round_num += 1;
        } else {
            round_num = 1;
        }

        let round_result = current_round.play_round();
        println!("round: {}, result: {}", round_num, round_result);
        score += round_result as u64;
    }

    println!("{score}");
}
