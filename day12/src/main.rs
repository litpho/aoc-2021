use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
};

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

fn part_one(input: HashMap<String, Node>) -> usize {
    traverse(&input, &[], "start", DefaultRouteFilter::new()).len()
}

fn part_two(input: HashMap<String, Node>) -> usize {
    traverse(&input, &[], "start", SkipOnceRouteFilter::new()).len()
}

fn traverse<RF: RouteFilter>(
    input: &HashMap<String, Node>,
    traject: &[&str],
    key: &str,
    route_filter: RF,
) -> Vec<String> {
    if key == "end" {
        return vec![format!("{},end", traject.join(","))];
    }

    let mut traject = traject.to_owned();
    traject.push(key);
    let mut route_filter = route_filter;
    route_filter.add(key);

    let node = input.get(key).unwrap();
    node.routes
        .iter()
        .filter(|route| route_filter.filter(route))
        .flat_map(|route| traverse(input, &traject, route, route_filter.clone()))
        .collect::<Vec<String>>()
}

trait RouteFilter: Clone {
    fn add(&mut self, key: &str);
    fn filter(&self, route: &str) -> bool;
}

#[derive(Clone)]
struct DefaultRouteFilter {
    keys: Vec<String>,
}

impl RouteFilter for DefaultRouteFilter {
    fn add(&mut self, key: &str) {
        if key == key.to_lowercase() {
            self.keys.push(key.to_string());
        }
    }

    fn filter(&self, route: &str) -> bool {
        !self.keys.contains(&route.to_string())
    }
}

impl DefaultRouteFilter {
    pub fn new() -> Self {
        DefaultRouteFilter { keys: vec![] }
    }
}

#[derive(Clone)]
struct SkipOnceRouteFilter {
    keys: HashSet<String>,
    wait_for_second: bool,
}

impl RouteFilter for SkipOnceRouteFilter {
    fn add(&mut self, key: &str) {
        if key == key.to_lowercase() && key != "start" {
            if self.keys.contains(key) {
                self.wait_for_second = false;
            }
            self.keys.insert(key.to_string());
        }
    }

    fn filter(&self, route: &str) -> bool {
        self.wait_for_second || !self.keys.contains(route)
    }
}

impl SkipOnceRouteFilter {
    pub fn new() -> Self {
        SkipOnceRouteFilter {
            keys: HashSet::new(),
            wait_for_second: true,
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    key: String,
    routes: HashSet<String>,
}

impl Node {
    pub fn new(key: &str) -> Self {
        let key = String::from(key);
        let routes = HashSet::new();
        Node { key, routes }
    }

    pub fn add_route(&mut self, route: &str) {
        if self.key == "end" {
            return;
        }
        if route == "start" {
            return;
        }
        self.routes.insert(String::from(route));
    }
}

fn parse(input: &str) -> IResult<&str, HashMap<String, Node>> {
    map(separated_list1(line_ending, parse_line), |v| {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        v.into_iter().for_each(|(from, to)| {
            nodes
                .entry(String::from(from))
                .or_insert_with(|| Node::new(from))
                .add_route(to);
            nodes
                .entry(String::from(to))
                .or_insert_with(|| Node::new(to))
                .add_route(from);
        });
        nodes
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("-"), alpha1)(input)
}

fn read_input() -> Result<HashMap<String, Node>> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(&buf).ok().unwrap();

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let count = part_one(input);

        assert_eq!(4304, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(input);

        assert_eq!(118242, count);

        Ok(())
    }
}
