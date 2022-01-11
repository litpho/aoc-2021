use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{
    cmp::{max, min},
    fs,
    io::Read,
    str::FromStr,
    time::Instant,
};

fn main() -> Result<()> {
    let lines = read_input()?;
    let start = Instant::now();

    let count = part_one(&lines);
    println!("Part one count: {}", count);

    let lap = Instant::now();
    println!("Time spent: {}ms", lap.duration_since(start).as_millis());

    let count = part_two(&lines);
    println!("Part two count: {}", count);

    let end = Instant::now();
    println!("Time spent: {}ms", end.duration_since(lap).as_millis());

    Ok(())
}

fn part_one(lines: &[Line]) -> usize {
    let mut grid = create_grid();

    fill_grid(
        &mut grid,
        lines.iter().filter(|line| line.horizontal || line.vertical),
    );

    count_grid(&grid)
}

fn part_two(lines: &[Line]) -> usize {
    let mut grid = create_grid();

    fill_grid(&mut grid, lines.iter());

    count_grid(&grid)
}

fn create_grid() -> Vec<usize> {
    let mut grid: Vec<usize> = Vec::new();
    (0..1_000_000).for_each(|_| grid.push(0));
    grid
}

fn fill_grid<'a, I: IntoIterator<Item = &'a Line>>(grid: &mut [usize], lines: I) {
    for line in lines {
        line.points()
            .iter()
            .for_each(|(x, y)| grid[y * 1000 + x] += 1);
    }
}

fn count_grid(grid: &[usize]) -> usize {
    grid.iter().filter(|val| val >= &&2).count()
}

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    horizontal: bool,
    vertical: bool,
}

impl Line {
    pub fn new(p1: (usize, usize), p2: (usize, usize)) -> Self {
        Line {
            x1: p1.0,
            y1: p1.1,
            x2: p2.0,
            y2: p2.1,
            horizontal: p1.1 == p2.1,
            vertical: p1.0 == p2.0,
        }
    }

    pub fn points(&self) -> Vec<(usize, usize)> {
        if self.horizontal {
            let range = Self::calculate_range(self.x1, self.x2);
            return range.map(|x| (x, self.y1)).collect::<Vec<(usize, usize)>>();
        }
        if self.vertical {
            let range = Self::calculate_range(self.y1, self.y2);
            return range.map(|y| (self.x1, y)).collect::<Vec<(usize, usize)>>();
        }
        let hor_range = Self::calculate_range(self.x1, self.x2);
        let ver_range = Self::calculate_range(self.y1, self.y2);
        hor_range.zip(ver_range).collect::<Vec<(usize, usize)>>()
    }

    fn calculate_range(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
        let range = min(a, b)..=max(a, b);
        if a > b {
            return Box::new(range.rev());
        }
        Box::new(range)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    map(
        separated_pair(parse_point, tag(" -> "), parse_point),
        |(p1, p2)| Line::new(p1, p2),
    )(input)
}

fn parse_point(input: &str) -> IResult<&str, (usize, usize)> {
    map(separated_pair(digit1, tag(","), digit1), |(x, y)| {
        (usize::from_str(x).unwrap(), usize::from_str(y).unwrap())
    })(input)
}

fn read_input() -> Result<Vec<Line>> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, lines) = parse(&buf).ok().unwrap();

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let lines = read_input()?;

        let count = part_one(&lines);

        assert_eq!(7142, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let lines = read_input()?;

        let count = part_two(&lines);

        assert_eq!(20012, count);

        Ok(())
    }
}
