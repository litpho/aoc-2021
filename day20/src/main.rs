use anyhow::Result;
use nom::{
    character::complete::line_ending,
    character::complete::one_of,
    combinator::map,
    multi::separated_list1,
    multi::{count, many1},
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashSet, fs, io::Read};

fn main() -> Result<()> {
    let (key, grid) = read_input()?;

    let (took, result) = took::took(|| part_one(key.clone(), grid.clone()));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(key, grid));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(key: Vec<bool>, mut grid: Grid) -> usize {
    (0..2).for_each(|_| grid.enhance(&key));

    // grid.draw();

    grid.pixels.len()
}

fn part_two(key: Vec<bool>, mut grid: Grid) -> usize {
    (0..50).for_each(|_| grid.enhance(&key));

    // grid.draw();

    grid.pixels.len()
}

#[derive(Clone)]
struct Grid {
    pixels: HashSet<(isize, isize)>,
    max_x: isize,
    max_y: isize,
    attempt: usize,
    x_padding: isize,
    y_padding: isize,
}

impl Grid {
    pub fn new(pixels: HashSet<(isize, isize)>, max_x: isize, max_y: isize) -> Self {
        Grid {
            pixels,
            max_x,
            max_y,
            attempt: 0,
            x_padding: 50,
            y_padding: 50,
        }
    }

    fn enhance(&mut self, key: &[bool]) {
        self.attempt += 1;
        let def_value = if self.attempt % 2 == 0 { "1" } else { "0" };
        let default = if *key.first().unwrap() {
            def_value
        } else {
            "0"
        };
        let mut new_image: HashSet<(isize, isize)> = HashSet::new();
        for y in -self.y_padding..self.max_y + self.y_padding {
            for x in -self.x_padding..self.max_x + self.x_padding {
                let idx = self.calc_bin(x, y, default);
                let res = *key.get(idx).unwrap();
                if res {
                    new_image.insert((x, y));
                }
            }
        }

        self.pixels = new_image;
    }

    fn calc_bin(&self, x: isize, y: isize, default: &str) -> usize {
        let mut bin_string = String::new();
        for cy in y - 1..=y + 1 {
            for cx in x - 1..=x + 1 {
                let val = self.pixels.get(&(cx, cy)).map_or("0", |_| "1");
                let val = if cy < -self.y_padding
                    || cx < -self.x_padding
                    || cx > self.max_x + self.x_padding - 1
                    || cy > self.max_y + self.y_padding - 1
                {
                    default
                } else {
                    val
                };
                bin_string.push_str(val);
            }
        }

        usize::from_str_radix(&bin_string, 2).unwrap()
    }

    #[allow(dead_code)]
    fn draw(&self) {
        for y in -self.y_padding..self.max_y + self.y_padding {
            for x in -self.x_padding..self.max_x + self.x_padding {
                let s = if self.pixels.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                print!("{s}");
            }
            println!();
        }
    }
}

fn parse(input: &[u8]) -> IResult<&[u8], (Vec<bool>, Grid)> {
    separated_pair(parse_line, count(line_ending, 2), parse_image)(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<bool>> {
    many1(map(one_of("#."), |c| c == '#'))(input)
}

fn parse_image(input: &[u8]) -> IResult<&[u8], Grid> {
    map(separated_list1(line_ending, parse_line), |v| {
        let mut pixels: HashSet<(isize, isize)> = HashSet::new();
        for (y, y_vec) in v.iter().enumerate() {
            for (x, val) in y_vec.iter().enumerate() {
                if *val {
                    pixels.insert((x as isize, y as isize));
                }
            }
        }
        Grid::new(pixels, v.get(0).unwrap().len() as isize, v.len() as isize)
    })(input)
}

fn read_input() -> Result<(Vec<bool>, Grid)> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, (key, grid)) = parse(buf.as_bytes()).expect("Parse failure");

    Ok((key, grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let (key, grid) = read_input()?;

        let result = part_one(key, grid);

        assert_eq!(5765, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (key, grid) = read_input()?;

        let result = part_two(key, grid);

        assert_eq!(18509, result);

        Ok(())
    }
}
