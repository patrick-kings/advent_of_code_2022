fn main() {
    let games = read_puzzle_input_from_file().unwrap();

    total_score_part_1(&games).unwrap();

    total_score_part_2(&games).unwrap();
}

// X 'lose', Y 'draw', Z 'win'
fn total_score_part_2(games: &Vec<Game>) -> Result<(), Box<dyn std::error::Error>> {
    let mut total_player_score = 0;

    for game in games {
        //
        let opponent: i32 = match game.opponent {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _other => 0,
        };

        let game_score = match game.player {
            // loss
            'X' => match opponent {
                1 => 3 + 0,
                2 => 1 + 0,
                3 => 2 + 0,
                _other => 0,
            },
            // draw
            'Y' => opponent + 3,
            // win
            'Z' => match opponent {
                1 => 2 + 6,
                2 => 3 + 6,
                3 => 1 + 6,
                _other => 0,
            },
            _other => 0,
        };
        total_player_score += game_score;
    }
    println!("total player score part 2 is {}", total_player_score);
    Ok(())
}

fn total_score_part_1(games: &Vec<Game>) -> Result<(), Box<dyn std::error::Error>> {
    let mut total_player_score = 0;

    for game in games {
        let mut game_score = 0;

        //
        let opponent: i32 = match game.opponent {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _other => 0,
        };

        let player = match game.player {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _other => 0,
        };

        if opponent == 0 || player == 0 {
            continue;
        }

        // draw
        if opponent == player {
            game_score = 3 + player;
        } else
        // win
        if player > opponent {
            // loss
            if player == 3 && opponent == 1 {
                game_score = player;
            } else {
                // win
                game_score = player + 6;
            }
        } else
        // loss
        if player < opponent {
            // win
            if opponent == 3 && player == 1 {
                game_score = player + 6;
            } else {
                // loss
                game_score = player;
            }
        }

        total_player_score += game_score;
    }

    println!("total player score part 1 is {}", total_player_score);

    Ok(())
}

// A rock, Y paper,  B paper,  X rock, C scissors, Z scissors
// your total score is the sum of your scores for each round.
// 1 for rock, 2 for paper, 3 for scissors
fn read_puzzle_input_from_file() -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("rock_paper_scissors.txt")?;

    let mut games = Vec::<Game>::new();

    let mut i: usize = 0;

    for line in contents.lines() {
        let ln: Vec<char> = line.chars().collect();

        games.push(Game {
            index: i,
            opponent: ln[0],
            player: ln[2],
        });

        i += 1;
    }

    println!("first index of puzzle data is '{:?}'", games[0]);

    Ok(games)
}

#[derive(Debug)]
pub struct Game {
    pub index: usize,
    pub opponent: char,
    pub player: char,
}
