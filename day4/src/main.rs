use core::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);
    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

#[derive(Debug)]
struct Group {
    low1: i32,
    high1: i32,
    low2: i32,
    high2: i32,
}

impl Group {
    fn contains(&self) -> bool {
        if &self.low1 >= &self.low2 && &self.high1 <= &self.high2 { return true; }
        if &self.low2 >= &self.low1 && &self.high2 <= &self.high1 { return true; }
        false
    }
    fn overlaps(&self) -> bool {
        if &self.low1 > &self.high2 || &self.high1 < &self.low2 { return false; }
        true
    }
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g: Vec<&str> = s.split(['-', ',']).collect();
        let low1 = i32::from_str(g[0]).unwrap_or(0);
        let high1 = i32::from_str(g[1]).unwrap_or(0);
        let low2 = i32::from_str(g[2]).unwrap_or(0);
        let high2 = i32::from_str(g[3]).unwrap_or(0);
        Ok(Group { low1, high1, low2, high2 })
    }
}

fn breakfast(input: &str) -> usize {
    let groups: Vec<Group> = input
        .trim()
        .split("\n")
        .map(|line| Group::from_str(line).unwrap())
        .collect();
    let contains: Vec<Group> = groups.into_iter().filter(|x| x.contains()).collect();
    contains.len()
}

fn lunch(input: &str) -> usize {
    let groups: Vec<Group> = input
        .trim()
        .split("\n")
        .map(|line| Group::from_str(line).unwrap())
        .collect();
    let overlaps: Vec<Group> = groups.into_iter().filter(|x| x.overlaps()).collect();
    overlaps.len()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 584);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 933);
    }
}