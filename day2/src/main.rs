use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);
    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

#[derive(Debug)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Hand(Symbol, Symbol);

struct Game(Symbol, Outcome);

fn score_game(game: &Game) -> i32 {
    match game.0 {
        Symbol::Rock => match game.1 {
            Outcome::Draw => 3 + 1,
            Outcome::Win => 6 + 2,
            Outcome::Loss => 0 + 3,
        },
        Symbol::Paper => match game.1 {
            Outcome::Loss => 0 + 1,
            Outcome::Draw => 3 + 2,
            Outcome::Win => 6 + 3,
        },
        Symbol::Scissors => match game.1 {
            Outcome::Win => 6 + 1,
            Outcome::Loss => 0 + 2,
            Outcome::Draw => 3 + 3,
        },
    }
}

fn score_hand(hand: &Hand) -> i32 {
    match hand.0 {
        Symbol::Rock => match hand.1 {
            Symbol::Rock => 3 + 1,
            Symbol::Paper => 6 + 2,
            Symbol::Scissors => 0 + 3,
        },
        Symbol::Paper => match hand.1 {
            Symbol::Rock => 0 + 1,
            Symbol::Paper => 3 + 2,
            Symbol::Scissors => 6 + 3,
        },
        Symbol::Scissors => match hand.1 {
            Symbol::Rock => 6 + 1,
            Symbol::Paper => 0 + 2,
            Symbol::Scissors => 3 + 3,
        },
    }
}

fn breakfast(input: &str) -> i32 {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let l: Vec<&str> = line.split(" ").into_iter().collect();
            Hand(Symbol::from_str(l[0]).unwrap(),
                 Symbol::from_str(l[1]).unwrap())
        })
        .collect();

    hands.iter().fold(0, |sum, x| sum + score_hand(x))
}

fn lunch(input: &str) -> i32 {
    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            let l: Vec<&str> = line.split(" ").into_iter().collect();
            Game(Symbol::from_str(l[0]).unwrap(),
                 Outcome::from_str(l[1]).unwrap())
        })
        .collect();

    games.iter().fold(0, |sum, x| sum + score_game(x))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 14264);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 12382);
    }
}
