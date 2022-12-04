use anyhow::Result;
use hierarchical_pathfinding::{prelude::ManhattanNeighborhood, PathCache, PathCacheConfig};
use nom::{
    bytes::complete::take_while1, character::complete::line_ending, character::is_digit,
    combinator::map, combinator::map_res, multi::separated_list1, IResult,
};
use std::{fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one_lib(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two_lib(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

// fn part_one(grid: &Grid) -> usize {
//     traverse_iteration(grid) as usize
// }

fn part_one_lib(grid: &Grid) -> usize {
    traverse_lib(grid)
}

fn part_two_lib(grid: &Grid) -> usize {
    let grid = grid.multiply_by_5();

    println!("{},{}", grid.max_x, grid.max_y);

    traverse_lib(&grid)
}

// fn traverse_iteration(grid: &Grid) -> i32 {
//     let route_start: Vec<(usize, usize, i32)> = vec![(0, 0, 0)];
//     // initialize lowest_count with diagonal. WARNING: only works with square grids
//     let mut lowest_count: i32 = (0..=grid.max_x).map(|i| grid.cell(i, i).2).sum::<i32>()
//         + (0..grid.max_x).map(|i| grid.cell(i, i + 1).2).sum::<i32>();
//
//     let mut results: Vec<Vec<(usize, usize, i32)>> = Vec::new();
//     let mut routes: VecDeque<(Vec<(usize, usize, i32)>, HashSet<(usize, usize)>)> = VecDeque::new();
//     routes.push_back((route_start, HashSet::new()));
//     while !routes.is_empty() {
//         let (route, visited) = routes.pop_front().unwrap();
//         let route = route.clone();
//
//         let mut visited = visited.clone();
//         let (x, y, last_val) = route.last().unwrap();
//         visited.insert((*x, *y));
//         for (a, b) in grid.get_neighbours(*x, *y) {
//             if visited.contains(&(a, b)) {
//                 continue;
//             }
//
//             let (a, b, val) = grid.cell(a, b);
//
//             if *last_val + val > lowest_count {
//                 continue;
//             }
//
//             if a > grid.max_x - 2 || b > grid.max_y - 2 {
//                 println!("{a},{b} - {}: {lowest_count}", routes.len());
//             }
//
//             // println!("{x},{y}:{a},{b} ({lowest_count},{}) - {:?}", routes.len(), route);
//
//             if a == grid.max_x && b == grid.max_y {
//                 println!("Bingo! {}: {lowest_count} < {}", routes.len(), *last_val + val);
//                 if lowest_count > *last_val + val {
//                     lowest_count = *last_val + val;
//                 }
//
//                 let mut new_route = route.to_vec();
//                 new_route.push((a, b, *last_val + val));
//                 results.push(new_route);
//             } else {
//                 let mut new_route = route.to_vec();
//                 new_route.push((a, b, *last_val + val));
//                 let mut new_visited = visited.to_owned();
//                 new_visited.insert((a, b));
//                 routes.push_front((new_route, new_visited));
//             }
//         }
//     }
//
//     lowest_count
// }

fn traverse_lib(grid: &Grid) -> usize {
    let pathfinding = PathCache::new(
        (grid.max_x + 1, grid.max_y + 1),
        |(x, y)| grid.cell(x, y).2 as isize,
        ManhattanNeighborhood::new(grid.max_x + 1, grid.max_y + 1),
        PathCacheConfig::with_chunk_size(1),
    );

    let start = (0, 0);
    let goal = (grid.max_x, grid.max_y);

    // find_path returns Some(Path) on success
    let path = pathfinding.find_path(start, goal, |(x, y)| grid.cell(x, y).2 as isize);

    if let Some(path) = path {
        let cost =
            path.cost() - grid.cell(0, 0).2 as usize + grid.cell(grid.max_x, grid.max_y).2 as usize;
        println!("Number of steps: {}", path.length());
        println!("Total Cost: {cost}");
        // for (x, y) in path {
        //     println!("Go to {x}, {y}");
        // }
        return cost;
    }

    0
}

struct Grid {
    content: Vec<Vec<i32>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    pub fn new(content: Vec<Vec<i32>>) -> Self {
        let max_x = content.get(0).unwrap().len() - 1;
        let max_y = content.len() - 1;
        Grid {
            content,
            max_x,
            max_y,
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> (usize, usize, i32) {
        (x, y, *self.content.get(y).unwrap().get(x).unwrap())
    }

    // pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    //     let mut result: Vec<(usize, usize)> = Vec::new();
    //     // println!("{x},{y}: {} > {} = {}", self.max_y - y, self.max_x - x, self.max_y - y > self.max_x - x);
    //     if x < self.max_x {
    //         result.push((x + 1, y));
    //     }
    //     if y < self.max_y {
    //         result.push((x, y + 1));
    //     }
    //     if self.max_y - y > self.max_x - x {
    //         // try for a diagonal from top left to bottom right as much as possible, favouring x over y
    //         result.reverse();
    //     }
    //     if x > 0 {
    //         result.push((x - 1, y));
    //     }
    //     if y > 0 {
    //         result.push((x, y - 1));
    //     }
    //     result
    // }

    pub fn multiply_by_5(&self) -> Self {
        let mut result: Vec<Vec<i32>> = Vec::new();

        for y in 0..5 {
            for row in &self.content {
                let mut vec: Vec<i32> = Vec::new();
                for x in 0..5 {
                    for col in row {
                        let mut val = x + y + col;
                        if val > 9 {
                            val -= 9;
                        }
                        vec.push(val);
                    }
                }
                result.push(vec);
            }
        }

        Grid::new(result)
    }
}

fn parse(input: &[u8]) -> IResult<&[u8], Grid> {
    map(separated_list1(line_ending, parse_line), |content| {
        Grid::new(content)
    })(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    map_res(take_while1(is_digit), |a: &[u8]| {
        a.iter()
            .map(|c| (c - b'0').to_string().parse::<i32>())
            .collect()
    })(input)
}

fn read_input() -> Result<Grid> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(buf.as_bytes()).expect("Parse failure");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let result = part_one_lib(&input);

        assert_eq!(503, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two_lib(&input);

        assert_eq!(2853, count);

        Ok(())
    }
}
