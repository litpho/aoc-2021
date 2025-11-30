use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    character::complete,
    combinator::map,
    multi::{count, many1, many_till},
    sequence::{pair, preceded},
    AsChar, IResult, Parser,
};
use std::{fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &Packet) -> u16 {
    input
        .flatten()
        .iter()
        .map(|p| p.version as u16)
        .sum::<u16>()
}

fn part_two(input: &Packet) -> u64 {
    input.content
}

#[derive(Debug)]
struct Packet {
    version: u8,
    content: u64,
    subpackets: Vec<Packet>,
}

impl Packet {
    pub fn from_content(version: u8, content: u64) -> Self {
        Packet {
            version,
            content,
            subpackets: Vec::new(),
        }
    }

    pub fn from_subpackets(version: u8, type_id: u8, subpackets: Vec<Packet>) -> Self {
        let content = match type_id {
            0 => subpackets.iter().map(|s| s.content).sum::<u64>(),
            1 => subpackets.iter().map(|s| s.content).product::<u64>(),
            2 => subpackets.iter().map(|s| s.content).min().unwrap(),
            3 => subpackets.iter().map(|s| s.content).max().unwrap(),
            5 => {
                u64::from(subpackets.first().unwrap().content > subpackets.get(1).unwrap().content)
            }
            6 => {
                u64::from(subpackets.first().unwrap().content < subpackets.get(1).unwrap().content)
            }
            7 => {
                u64::from(subpackets.first().unwrap().content == subpackets.get(1).unwrap().content)
            }
            _ => 0,
        };

        Packet {
            version,
            content,
            subpackets,
        }
    }

    pub fn flatten(&self) -> Vec<&Packet> {
        let vec = vec![self];
        let flattened_subpackets = self
            .subpackets
            .iter()
            .flat_map(|s| s.flatten())
            .collect::<Vec<&Packet>>();
        [vec, flattened_subpackets].concat()
    }
}

fn parse(input: &str) -> IResult<&str, Packet> {
    alt((parse_packet_4, parse_packet_not_4)).parse(input)
}

fn parse_packet_4(input: &str) -> IResult<&str, Packet> {
    let (input, version) = parse_version(input)?;
    let (input, content) = preceded(tag("100"), parse_literal).parse(input)?;

    let packet = Packet::from_content(version, content);

    Ok((input, packet))
}

fn parse_packet_not_4(input: &str) -> IResult<&str, Packet> {
    let (input, version) = parse_version(input)?;
    let (input, type_id) = parse_type_id(input)?;
    let (input, length_type_id) = take(1usize)(input)?;
    let (input, subpackets) = if length_type_id == "0" {
        let (input, total_subpacket_length) =
            map(take(15usize), |tsl| usize::from_str_radix(tsl, 2).unwrap()).parse(input)?;
        let (input, sub_input) = take(total_subpacket_length)(input)?;
        let (_, subpackets) = many1(parse).parse(sub_input)?;
        (input, subpackets)
    } else {
        let (input, number_of_subpackets) =
            map(take(11usize), |tsl| usize::from_str_radix(tsl, 2).unwrap()).parse(input)?;
        count(parse, number_of_subpackets).parse(input)?
    };

    let packet = Packet::from_subpackets(version, type_id, subpackets);

    Ok((input, packet))
}

fn parse_version(input: &str) -> IResult<&str, u8> {
    parse_three_bits(input)
}

fn parse_type_id(input: &str) -> IResult<&str, u8> {
    parse_three_bits(input)
}

fn parse_three_bits(input: &str) -> IResult<&str, u8> {
    map(take(3usize), |c: &str| u8::from_str_radix(c, 2).unwrap()).parse(input)
}

fn parse_literal(input: &str) -> IResult<&str, u64> {
    map(
        many_till(parse_literal_part, parse_literal_end),
        |(v, s)| {
            let mut buf = v.join("");
            buf.push_str(s);
            u64::from_str_radix(&buf, 2).unwrap()
        },
    )
    .parse(input)
}

fn parse_literal_part(input: &str) -> IResult<&str, &str> {
    map(pair(complete::char('1'), take(4usize)), |(_, b)| b).parse(input)
}

fn parse_literal_end(input: &str) -> IResult<&str, &str> {
    map(pair(complete::char('0'), take(4usize)), |(_, b)| b).parse(input)
}

fn parse_hex_as_binary(input: &[u8]) -> IResult<&[u8], String> {
    map(parse_arr_as_hex, |s| {
        s.iter()
            .map(|c| {
                let val = u8::from_str_radix(&c.as_char().to_string(), 16)
                    .ok()
                    .unwrap();
                format!("{:04b}", val)
            })
            .collect::<Vec<String>>()
            .join("")
    })
    .parse(input)
}

fn parse_arr_as_hex(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(|c: u8| c.is_hex_digit())(input)
}

fn read_input() -> Result<Packet> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse_hex_as_binary(buf.as_bytes()).expect("Parse hex failure");
    let (_, input) = parse(input.as_str()).expect("Parse failure");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let result = part_one(&input);

        assert_eq!(913, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let result = part_two(&input);

        assert_eq!(1510977819698, result);

        Ok(())
    }
}
