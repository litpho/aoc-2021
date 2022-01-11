use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{eof, map},
    multi::{count, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{collections::HashMap, fs, io::Read, ops::Rem, time::Instant};

fn main() -> Result<()> {
    let start = Instant::now();
    let (called_numbers, mut cards) = read_input()?;
    let (called_number, card) = part_one(&called_numbers, &mut cards).unwrap();

    let card_sum = card.sum();
    println!("called_number: {}", called_number);
    println!("Sum: {}", card_sum);
    println!("Result: {}", (called_number as u32) * card_sum);

    let lap = Instant::now();
    println!("Time spent: {}ms", lap.duration_since(start).as_millis());

    let (called_numbers, mut cards) = read_input()?;
    let (called_number, card) = part_two(&called_numbers, &mut cards).unwrap();
    let card_sum = card.sum();
    println!("called_number: {}", called_number);
    println!("Sum: {}", card_sum);
    println!("Result: {}", (called_number as u32) * card_sum);

    let end = Instant::now();
    println!("Time spent: {}ms", end.duration_since(lap).as_millis());

    Ok(())
}

fn part_one(called_numbers: &[u8], cards: &mut [Card]) -> Option<(u8, Card)> {
    for called_number in called_numbers {
        for card in cards.iter_mut() {
            if let Some(pos) = card.mark(called_number) {
                if card.bingo(pos) {
                    return Some((*called_number, card.clone()));
                }
            }
        }
    }

    None
}

fn part_two(called_numbers: &[u8], cards: &mut [Card]) -> Option<(u8, Card)> {
    let mut won_cards: Vec<Card> = Vec::new();
    for called_number in called_numbers {
        for card in cards.iter_mut() {
            if !won_cards.contains(card) {
                if let Some(pos) = card.mark(called_number) {
                    if card.bingo(pos) {
                        won_cards.push(card.clone());
                        if won_cards.len() == 100 {
                            return Some((*called_number, card.clone()));
                        }
                    }
                }
            }
        }
    }

    None
}

#[derive(Clone, Eq, PartialEq)]
struct Card {
    numbers: Vec<u8>,
    marked: HashMap<usize, bool>,
}

impl Card {
    pub fn new(numbers: Vec<u8>) -> Self {
        let marked = (0..25)
            .map(|pos| (pos, false))
            .collect::<HashMap<usize, bool>>();
        Card { numbers, marked }
    }

    pub fn mark(&mut self, number: &u8) -> Option<usize> {
        if let Some(position) = self.numbers.iter().position(|x| x == number) {
            self.marked.insert(position, true);
            return Some(position);
        }

        None
    }

    pub fn bingo(&self, position: usize) -> bool {
        let horizontal = (position.div_euclid(5) * 5 + 1..position.div_euclid(5) * 5 + 5)
            .map(|pos| self.marked[&pos])
            .all(|mark| mark);
        let rem = position.rem(5);
        let vertical = (0..5)
            .map(|pos| self.marked[&(pos * 5 + rem)])
            .all(|mark| mark);

        horizontal || vertical
    }

    pub fn sum(&self) -> u32 {
        self.numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| !*self.marked.get(i).unwrap())
            .map(|(_, num)| *num as u32)
            .sum()
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<u8>, Vec<Card>)> {
    separated_pair(parse_called_numbers, line_ending, parse_cards)(input)
}

fn parse_called_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    map(
        terminated(separated_list1(tag(","), digit1), line_ending),
        |nums| {
            nums.into_iter()
                .map(|num: &str| num.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        },
    )(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(count(parse_card_line, 5), |x| {
        let numbers = x.into_iter().flatten().collect::<Vec<u8>>();
        Card::new(numbers)
    })(input)
}

fn parse_card_line(input: &str) -> IResult<&str, Vec<u8>> {
    map(
        terminated(separated_list1(space1, digit1), alt((line_ending, eof))),
        |numbers: Vec<&str>| {
            numbers
                .iter()
                .map(|number| number.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        },
    )(input)
}

fn read_input() -> Result<(Vec<u8>, Vec<Card>)> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, (called_numbers, cards)) = parse(&buf).ok().unwrap();

    Ok((called_numbers, cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let (called_numbers, mut cards) = read_input()?;

        let (called_number, card) = part_one(&called_numbers, &mut cards).unwrap();

        assert_eq!(42, called_number);
        assert_eq!(782, card.sum());

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (called_numbers, mut cards) = read_input()?;

        let (called_number, card) = part_two(&called_numbers, &mut cards).unwrap();

        assert_eq!(20, called_number);
        assert_eq!(246, card.sum());

        Ok(())
    }
}
