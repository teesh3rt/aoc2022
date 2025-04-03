use std::ops::Not;

use nom::{
    IResult, Parser,
    bytes::tag,
    character::{anychar, complete::newline},
    combinator::opt,
    multi::many1,
    sequence::separated_pair,
};

const PUZZLE_INPUT: &str = include_str!("../inputs/two.pzlin");

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

// note(teesh): we do something smart here!
//              !move = the move that wins over this move
//
//              such that:
//              !Scissors = Rock
//              !Paper = Scissors
//              !Rock = Paper
impl Not for Play {
    type Output = Play;

    fn not(self) -> Self::Output {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

fn parse_play(input: &str) -> IResult<&str, Play> {
    let (input, char) = anychar(input)?;

    let play = match char {
        'A' => Play::Rock,
        'B' => Play::Paper,
        'C' => Play::Scissors,
        'X' => Play::Rock,
        'Y' => Play::Paper,
        'Z' => Play::Scissors,
        _ => panic!("Are you sure you got the puzzle input?"),
    };

    Ok((input, play))
}

#[derive(Clone)]
struct Round {
    you: Play,
    enemy: Play,
}

impl Round {
    fn calculate_your_score(&self) -> u32 {
        let move_score = self.you as u8;
        let outcome = if self.you == !self.enemy {
            6
        } else if self.you == self.enemy {
            3
        } else {
            0
        };

        (move_score + outcome) as u32
    }

    fn calculate_enemys_score(&self) -> u32 {
        let move_score = self.enemy as u8;
        let outcome = if self.enemy == !self.you {
            6
        } else if self.enemy == self.you {
            3
        } else {
            0
        };

        (move_score + outcome) as u32
    }
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, (enemy, you)) = separated_pair(parse_play, tag(" "), parse_play).parse(input)?;
    let (input, _) = opt(newline).parse(input)?;
    Ok((input, Round { you, enemy }))
}

fn parse_round_extra(input: &str) -> IResult<&str, Round> {
    let (input, (enemy, outcome)) = separated_pair(parse_play, tag(" "), anychar).parse(input)?;
    let (input, _) = opt(newline).parse(input)?;

    let you = match outcome {
        'X' => match enemy {
            Play::Rock => Play::Scissors,  // Lose to Rock -> Pick Scissors
            Play::Paper => Play::Rock,     // Lose to Paper -> Pick Rock
            Play::Scissors => Play::Paper, // Lose to Scissors -> Pick Paper
        },
        'Y' => enemy, // Draw with enemy -> Pick same move
        'Z' => match enemy {
            Play::Rock => Play::Paper,     // Win over Rock -> Pick Paper
            Play::Paper => Play::Scissors, // Win over Paper -> Pick Scissors
            Play::Scissors => Play::Rock,  // Win over Scissors -> Pick Rock
        },
        _ => panic!("Invalid outcome in input!"),
    };

    Ok((input, Round { you, enemy }))
}

pub struct Game(Vec<Round>);

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, rounds) = many1(parse_round).parse(input)?;
    Ok((input, Game(rounds)))
}

fn parse_game_extra(input: &str) -> IResult<&str, Game> {
    let (input, rounds) = many1(parse_round_extra).parse(input)?;
    Ok((input, Game(rounds)))
}

impl Game {
    fn calculate_your_score(&self) -> u32 {
        self.0
            .clone()
            .into_iter()
            .map(|round| round.calculate_your_score())
            .sum()
    }

    fn calculate_enemys_score(&self) -> u32 {
        self.0
            .clone()
            .into_iter()
            .map(|round| round.calculate_enemys_score())
            .sum()
    }
}

#[test]
fn test_parse_round() {
    let (input, output) = parse_round("A Y\n").unwrap();
    assert!(input.is_empty());
    assert_eq!(output.you, Play::Paper);
    assert_eq!(output.enemy, Play::Rock);
    assert_eq!(output.calculate_your_score(), 8);
    assert_eq!(output.calculate_enemys_score(), 1);

    let (input, output) = parse_round("C X").unwrap();
    assert!(input.is_empty());
    assert_eq!(output.you, Play::Rock);
    assert_eq!(output.enemy, Play::Scissors);
    assert_eq!(output.calculate_enemys_score(), 3);
    assert_eq!(output.calculate_your_score(), 7);
}

#[test]
fn test_parse_game() {
    let (input, game) = parse_game("A Y\nB X\nC Z").unwrap();
    assert!(input.is_empty());
    assert_eq!(game.calculate_enemys_score(), 15);
}

#[test]
fn test_first() {
    let (input, game) = parse_game(PUZZLE_INPUT).unwrap();
    assert!(input.is_empty());
    assert_eq!(game.calculate_your_score(), 14827)
}

#[test]
fn test_extra() {
    let (input, game) = parse_game_extra(PUZZLE_INPUT).unwrap();
    assert!(input.is_empty());
    assert_eq!(game.calculate_your_score(), 13889)
}
