use std::{collections::HashMap, fs, io::Read, ops::Rem};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete, complete::line_ending, complete::space1},
    combinator::{eof, map},
    multi::{count, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

fn main() -> Result<()> {
    let (called_numbers, mut cards) = read_input()?;

    let (took, result) = took::took(|| {
        let mut cards = cards.clone();
        part_one(&called_numbers, &mut cards)
    });
    let (called_number, card_sum) = result.expect("No result found");
    println!("called_number: {called_number}");
    println!("Sum: {card_sum}");
    println!("Result part one: {}", (called_number as u32) * card_sum);
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&called_numbers, &mut cards));
    let (called_number, card_sum) = result.expect("No result found");
    println!("called_number: {called_number}");
    println!("Sum: {card_sum}");
    println!("Result part two: {}", (called_number as u32) * card_sum);
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(called_numbers: &[u8], cards: &mut [Card]) -> Option<(u8, u32)> {
    for called_number in called_numbers {
        for card in cards.iter_mut() {
            if let Some(pos) = card.mark(called_number) {
                if card.bingo(pos) {
                    return Some((*called_number, card.sum()));
                }
            }
        }
    }

    None
}

fn part_two(called_numbers: &[u8], cards: &mut [Card]) -> Option<(u8, u32)> {
    let mut won_cards: Vec<Card> = Vec::new();
    for called_number in called_numbers {
        for card in cards.iter_mut() {
            if !won_cards.contains(card) {
                if let Some(pos) = card.mark(called_number) {
                    if card.bingo(pos) {
                        won_cards.push(card.clone());
                        if won_cards.len() == 100 {
                            return Some((*called_number, card.sum()));
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
    terminated(separated_list1(tag(","), complete::u8), line_ending)(input)
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
    terminated(
        separated_list1(space1, complete::u8),
        alt((line_ending, eof)),
    )(input)
}

fn read_input() -> Result<(Vec<u8>, Vec<Card>)> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, (called_numbers, cards)) = parse(&buf).expect("Parse failure");

    Ok((called_numbers, cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let (called_numbers, mut cards) = read_input()?;

        let (called_number, card_sum) =
            part_one(&called_numbers, &mut cards).expect("No result found");

        assert_eq!(42, called_number);
        assert_eq!(782, card_sum);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (called_numbers, mut cards) = read_input()?;

        let (called_number, card_sum) =
            part_two(&called_numbers, &mut cards).expect("No result found");

        assert_eq!(20, called_number);
        assert_eq!(246, card_sum);

        Ok(())
    }
}
