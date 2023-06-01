fn main() {
    let games = read_text().unwrap();

    total_score(&games).unwrap();
}

fn total_score(games: &Vec<Game>) -> Result<(), Box<dyn std::error::Error>> {
    let mut score = 0;

    for (_i, game) in games.iter().enumerate() {
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
            game_score = opponent + player;
        } else
        // win
        if player > opponent {
            // loss
            if opponent == 1 && player == 3 {
                game_score = player;
            } else {
                game_score = player + 6;
            }
        } else
        // loss
        if player < opponent {
            // win
            if opponent == 3 && player == 1 {
                game_score = player + 6;
            } else {
                game_score = player;
            }
        }

        println!(
            "score {} game_score {} opponent {} player {}",
            score, game_score, opponent, player
        );
        score += game_score;
    }

    println!("{}", score);

    Ok(())
}

// A rock, Y paper,  B paper,  X rock, C scissors, Z scissors
// your total score is the sum of your scores for each round.
// 1 for rock, 2 for paper, 3 for scissors
fn read_text() -> Result<Vec<Game>, Box<dyn std::error::Error>> {
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

    println!("{:?}", games[0]);

    Ok(games)
}

#[derive(Debug)]
pub struct Game {
    pub index: usize,
    pub opponent: char,
    pub player: char,
}
