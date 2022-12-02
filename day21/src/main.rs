use anyhow::Result;
use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::line_ending,
    combinator::map, multi::separated_list1, sequence::tuple, IResult,
};
use once_cell::sync::Lazy;
use std::{cell::RefCell, cmp::max, fs, io::Read, str::FromStr};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(input.clone()));
    println!("Result part one: {}", result);
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {}", result);
    println!("Time spent: {}", took);

    Ok(())
}

fn part_one(input: Vec<u8>) -> u32 {
    let mut one = Player::new(*input.first().unwrap());
    let mut two = Player::new(*input.get(1).unwrap());

    let die = Die::new();

    loop {
        one.move_pawn(die.roll() + die.roll() + die.roll());
        if one.score >= 1000 {
            return two.score * *die.amount.borrow() as u32;
        }
        two.move_pawn(die.roll() + die.roll() + die.roll());
        if two.score >= 1000 {
            return one.score * *die.amount.borrow() as u32;
        }
    }
}

fn part_two(input: Vec<u8>) -> u64 {
    let start = Universe::new(
        1,
        *input.first().unwrap(),
        *input.get(1).unwrap(),
        0,
        0,
        false,
        false,
    );
    let mut universes: Vec<Universe> = vec![start];
    let mut games_won_one = 0u64;
    let mut games_won_two = 0u64;

    while !universes.is_empty() {
        universes = universes
            .into_iter()
            .filter(|u| !u.won_one && !u.won_two)
            .flat_map(|u| u.spawn_one())
            .collect::<Vec<Universe>>();
        games_won_one += universes
            .iter()
            .filter(|u| u.won_one)
            .map(|u| u.amount)
            .sum::<u64>();

        universes = universes
            .into_iter()
            .filter(|u| !u.won_one && !u.won_two)
            .flat_map(|u| u.spawn_two())
            .collect::<Vec<Universe>>();
        games_won_two += universes
            .iter()
            .filter(|u| u.won_two)
            .map(|u| u.amount)
            .sum::<u64>();
    }

    max(games_won_one, games_won_two)
}

static DISTRIBUTION: Lazy<Vec<(u8, u8)>> =
    Lazy::new(|| vec![(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)]);

#[derive(Clone, Debug)]
struct Universe {
    amount: u64,
    positions: [u8; 2],
    scores: [u16; 2],
    won_one: bool,
    won_two: bool,
}

impl Universe {
    pub fn new(
        amount: u64,
        pos_one: u8,
        pos_two: u8,
        score_one: u16,
        score_two: u16,
        won_one: bool,
        won_two: bool,
    ) -> Self {
        Universe {
            amount,
            positions: [pos_one, pos_two],
            scores: [score_one, score_two],
            won_one,
            won_two,
        }
    }

    pub fn spawn_one(&self) -> Vec<Self> {
        DISTRIBUTION
            .iter()
            .map(|(n, spaces)| {
                let new_pos = Self::calc_pos(self.positions[0], *spaces);
                let new_score = self.scores[0] + new_pos as u16;
                let won_one = new_score >= 21;
                Universe::new(
                    self.amount * *n as u64,
                    new_pos,
                    self.positions[1],
                    new_score,
                    self.scores[1],
                    won_one,
                    false,
                )
            })
            .collect::<Vec<Self>>()
    }

    pub fn spawn_two(&self) -> Vec<Self> {
        DISTRIBUTION
            .iter()
            .map(|(n, spaces)| {
                let new_pos = Self::calc_pos(self.positions[1], *spaces);
                let new_score = self.scores[1] + new_pos as u16;
                let won_two = new_score >= 21;
                Universe::new(
                    self.amount * *n as u64,
                    self.positions[0],
                    new_pos,
                    self.scores[0],
                    new_score,
                    false,
                    won_two,
                )
            })
            .collect::<Vec<Self>>()
    }

    fn calc_pos(pos: u8, spaces: u8) -> u8 {
        ((pos - 1 + spaces) % 10) + 1
    }
}

struct Player {
    pos: u16,
    score: u32,
}

impl Player {
    pub fn new(pos: u8) -> Self {
        Player {
            pos: pos as u16,
            score: 0,
        }
    }

    pub fn move_pawn(&mut self, spaces: u16) {
        self.pos = ((self.pos - 1 + spaces) % 10) + 1;
        self.score += self.pos as u32;
    }
}

struct Die {
    next: RefCell<u16>,
    amount: RefCell<u16>,
}

impl Die {
    pub fn new() -> Self {
        Die {
            next: RefCell::new(100),
            amount: RefCell::new(0),
        }
    }

    pub fn roll(&self) -> u16 {
        let mut amount = self.amount.borrow_mut();
        *amount += 1;

        let mut next = self.next.borrow_mut();
        if next.gt(&99) {
            *next = 1;
        } else {
            *next += 1;
        }
        *next
    }
}

fn parse(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(line_ending, parse_player)(input)
}

fn parse_player(input: &str) -> IResult<&str, u8> {
    map(
        tuple((tag("Player "), digit1, tag(" starting position: "), digit1)),
        |(_, _, _, pos)| u8::from_str(pos).unwrap(),
    )(input)
}

fn read_input() -> Result<Vec<u8>> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(&buf).expect("Parse failure");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let result = part_one(input);

        assert_eq!(893700, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let result = part_two(input);

        assert_eq!(568867175661958, result);

        Ok(())
    }
}
