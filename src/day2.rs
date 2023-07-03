
#[derive(PartialEq)]
#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors
}


#[derive(PartialEq)]
#[derive(Debug)]
enum Result {
    Win,
    Loss,
    Draw
}

struct Round {
    mine: Hand,
    opponent: Hand
}

pub fn day2_part_a(contents: &str) -> i32 {
    parse_input(contents)
        .map(|round| score_round(round))
        .sum()
}

fn parse_input<'a>(contents: &'a str) -> impl Iterator<Item = Round> + 'a {
    contents
        .lines()
        .map(
        |line| {
            let mut chars = line.chars();
            let opponent = match chars.next() {
                Some('A') => Hand::Rock,
                Some('B') => Hand::Paper,
                Some('C') => Hand::Scissors,
                _ => panic!("Invalid opponent hand")
            };
            let _ = chars.next();
            let mine = match chars.next() {
                Some('X') => Hand::Rock,
                Some('Y') => Hand::Paper,
                Some('Z') => Hand::Scissors,
                _ => panic!("Invalid my hand")
            };
            Round { mine, opponent }
        }
        )
}

fn rps_is_win(round: Round) -> Result {
    match round {
        Round { mine: Hand::Rock, opponent: Hand::Scissors } => Result::Win,
        Round { mine: Hand::Paper, opponent: Hand::Rock } => Result::Win,
        Round { mine: Hand::Scissors, opponent: Hand::Paper } => Result::Win,
        Round { opponent: Hand::Rock, mine: Hand::Scissors } => Result::Loss,
        Round { opponent: Hand::Paper, mine: Hand::Rock } => Result::Loss,
        Round { opponent: Hand::Scissors, mine: Hand::Paper } => Result::Loss,
        _ => Result::Draw
    }
}

fn score_round(round: Round) -> i32 {
    const ROCK_SCORE: i32 = 1;
    const PAPER_SCORE: i32 = 2;
    const SCISSORS_SCORE: i32 = 3;

    const WIN_SCORE: i32 = 6;
    const DRAW_SCORE: i32 = 3;
    const LOSS_SCORE: i32 = 0;

    let hand_score = match round.mine {
        Hand::Rock => ROCK_SCORE,
        Hand::Paper => PAPER_SCORE,
        Hand::Scissors => SCISSORS_SCORE,
    };

    let outcome_score = match rps_is_win(round) {
        Result::Win => WIN_SCORE,
        Result::Loss => LOSS_SCORE,
        Result::Draw => DRAW_SCORE,
    };
    hand_score + outcome_score
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_day2_part_a() {
        let contents = "\
A Y
B X
C Z";
        assert_eq!(day2_part_a(contents), 15);
    }

    #[test]
    fn test_parse_input() {
        let contents = "\
A Y
B X
C Z";
        let result: Vec<Round> = parse_input(contents).collect();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].opponent, Hand::Rock);
        assert_eq!(result[0].mine, Hand::Paper);
        assert_eq!(result[1].opponent, Hand::Paper);
        assert_eq!(result[1].mine, Hand::Rock);
        assert_eq!(result[2].opponent, Hand::Scissors);
        assert_eq!(result[2].mine, Hand::Scissors);
    }

    #[test]
    fn test_rps_is_win() {
        assert_eq!(rps_is_win(Round { mine: Hand::Rock, opponent: Hand::Scissors }), Result::Win);
        assert_eq!(rps_is_win(Round { mine: Hand::Paper, opponent: Hand::Rock }), Result::Win);
        assert_eq!(rps_is_win(Round { mine: Hand::Scissors, opponent: Hand::Paper }), Result::Win);
        assert_eq!(rps_is_win(Round { opponent: Hand::Rock, mine: Hand::Scissors }), Result::Loss);
        assert_eq!(rps_is_win(Round { opponent: Hand::Paper, mine: Hand::Rock }), Result::Loss);
        assert_eq!(rps_is_win(Round { opponent: Hand::Scissors, mine: Hand::Paper }), Result::Loss);
        assert_eq!(rps_is_win(Round { opponent: Hand::Rock, mine: Hand::Rock }), Result::Draw);
        assert_eq!(rps_is_win(Round { opponent: Hand::Paper, mine: Hand::Paper }), Result::Draw);
        assert_eq!(rps_is_win(Round { opponent: Hand::Scissors, mine: Hand::Scissors }), Result::Draw);
    }

    #[test]
    fn test_score_round() {
        assert_eq!(score_round(Round { mine: Hand::Rock, opponent: Hand::Scissors }), 6 + 1);
        assert_eq!(score_round(Round { mine: Hand::Paper, opponent: Hand::Paper }), 3 + 2);
        assert_eq!(score_round(Round { mine: Hand::Scissors, opponent: Hand::Rock }), 0 + 3);
    }
}