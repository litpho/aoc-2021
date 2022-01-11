use anyhow::Result;
use nom::{
    bytes::complete::take_while1, character::complete::line_ending, character::is_digit,
    combinator::map, multi::separated_list1, IResult,
};
use std::{fs, io::Read, time::Instant};

fn main() -> Result<()> {
    let start = Instant::now();
    let mut input = read_input()?;
    let count = part_one(&mut input);
    println!("Part one count: {}", count);

    let lap = Instant::now();
    println!(
        "Time spent: {} milliseconds",
        lap.duration_since(start).as_millis()
    );

    let mut input = read_input()?;
    let count = part_two(&mut input);
    println!("Part two count: {}", count);

    let end = Instant::now();
    println!(
        "Time spent: {} milliseconds",
        end.duration_since(lap).as_millis()
    );

    Ok(())
}

fn part_one(grid: &mut Grid) -> i32 {
    (0..100).map(|_| step(grid)).sum::<i32>()
}

fn part_two(grid: &mut Grid) -> i32 {
    (1..).find(|_| step(grid) == 100).unwrap()
}

fn step(grid: &mut Grid) -> i32 {
    let mut flashed: Vec<(usize, usize)> = Vec::new();

    grid.inc_all_by_one();
    for x in 0..=grid.max_x {
        for y in 0..=grid.max_y {
            if flashed.contains(&(x, y)) {
                continue;
            }
            flash(grid, x, y, &mut flashed);
        }
    }

    flashed.iter().for_each(|(x, y)| *grid.cell(*x, *y) = 0);

    flashed.len() as i32
}

fn flash(grid: &mut Grid, x: usize, y: usize, flashed: &mut Vec<(usize, usize)>) {
    if grid.cell(x, y) <= &mut 9 {
        return;
    }

    flashed.push((x, y));

    let mut idx = 0;
    let mut neighbours_to_check: Vec<Vec<(usize, usize)>> = vec![grid.get_neighbours(x, y)];

    while neighbours_to_check.len() > idx {
        let mut neighbours: Vec<(usize, usize)> = Vec::new();
        for (x, y) in neighbours_to_check.get(idx).unwrap() {
            if flashed.contains(&(*x, *y)) {
                continue;
            }

            *grid.cell(*x, *y) += 1;
            if *grid.cell(*x, *y) > 9 {
                flashed.push((*x, *y));
                neighbours.append(&mut grid.get_neighbours(*x, *y));
            }
        }

        if !neighbours.is_empty() {
            // println!("Add neighbours: {}", neighbours.len());
            neighbours_to_check.push(neighbours);
        }

        idx += 1;
    }
}

struct Grid {
    content: Vec<Vec<i32>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    pub fn new(content: Vec<Vec<i32>>) -> Self {
        let max_x = content.len() - 1;
        let max_y = content.get(0).unwrap().len() - 1;
        Grid {
            content,
            max_x,
            max_y,
        }
    }

    pub fn inc_all_by_one(&mut self) {
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                *self.content.get_mut(y).unwrap().get_mut(x).unwrap() += 1;
            }
        }
    }

    pub fn cell(&mut self, x: usize, y: usize) -> &mut i32 {
        self.content.get_mut(y).unwrap().get_mut(x).unwrap()
    }

    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if y > 0 {
            if x > 0 {
                result.push((x - 1, y - 1));
            }
            result.push((x, y - 1));
            if x < self.max_x {
                result.push((x + 1, y - 1));
            }
        }
        if x > 0 {
            result.push((x - 1, y));
        }
        if x < self.max_x {
            result.push((x + 1, y));
        }
        if y < self.max_y {
            if x > 0 {
                result.push((x - 1, y + 1));
            }
            result.push((x, y + 1));
            if x < self.max_x {
                result.push((x + 1, y + 1));
            }
        }
        result
    }
}

fn parse(input: &[u8]) -> IResult<&[u8], Grid> {
    map(separated_list1(line_ending, parse_line), |content| {
        Grid::new(content)
    })(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    map(take_while1(is_digit), |a: &[u8]| {
        a.iter()
            .map(|c| (c - 48).to_string().parse::<i32>().unwrap())
            .collect()
    })(input)
}

fn read_input() -> Result<Grid> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(buf.as_bytes()).ok().unwrap();

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let mut input = read_input()?;

        let count = part_one(&mut input);

        assert_eq!(1793, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let mut input = read_input()?;

        let count = part_two(&mut input);

        assert_eq!(247, count);

        Ok(())
    }
}
