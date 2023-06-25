use std::fs::{ self };


#[derive(PartialEq)]
enum MatchRes {
    Win, Draw, Loose
}

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Rock, Paper, Scissor
}

#[derive(Debug)]
struct GamePlay {
    opp: Move,
    me: Move,
}

impl GamePlay {
    fn calc_my_score(&self) -> u32 {
        let game_score = match self.me.play_with(&self.opp) {
            MatchRes::Win => 6,
            MatchRes::Draw => 3,
            MatchRes::Loose => 0,
        };
        return self.me.item_score() + game_score;
    }
}

impl From<&str> for GamePlay {
    fn from(s: &str) -> GamePlay {

        let (opp, me) = s.split_once(" ")
            .expect("invalud format !");

        let opp = match opp {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            _ => panic!("unrecognized move played !"),
        };

        // change here for part 1 and part 2
        let me = match me {
            "X" => match opp { // loose
                Move::Rock => Move::Scissor,
                Move::Paper => Move::Rock,
                Move::Scissor => Move::Paper,
            },
            "Y" => opp.clone(), // draw 
            "Z" => match opp { // win
                Move::Rock =>  Move::Paper,
                Move::Paper => Move::Scissor,
                Move::Scissor => Move::Rock,
            },
            _ => panic!("unrecognized move played !"),
        };

        return GamePlay {
            opp,
            me
        }
    }
}

impl Move {

    fn item_score(&self) -> u32 {
        return match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        };
    }

    fn play_with(&self, other: &Self) -> MatchRes {
        if *self == *other {
            return MatchRes::Draw;
        }
        else if *self == Self::Rock && *other == Self::Scissor {
            return MatchRes::Win;
        }
        else if *self == Self::Paper && *other == Self::Rock {
            return MatchRes::Win;
        }
        else if *self == Self::Scissor && *other == Self::Paper {
            return MatchRes::Win;
        }
        return MatchRes::Loose;
    }

}


fn main() {
    
    let data = fs::read_to_string("input.txt").unwrap();

    let total_score: u32 = data
        .lines()
        .map(|entry| entry.into())
        .map(|roll: GamePlay| roll.calc_my_score())
        .sum();

    println!("My total final score is {} ", total_score);

}