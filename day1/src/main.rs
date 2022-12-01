fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("max calories of an elf: {}", result_breakfast);
    let result_lunch = lunch(input);
    println!("calories of top 3 elves: {}", result_lunch);
}

fn breakfast(input: &str) -> i32 {
    let elves: Vec<i32> = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|calories| calories.parse::<i32>().unwrap()).sum())
        .collect();
    *elves.iter().max().unwrap()
}

fn lunch(input: &str) -> i32 {
    let mut elves: Vec<i32> = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|calories| calories.parse::<i32>().unwrap()).sum())
        .collect();

    elves.sort();
    let len = elves.len();
    let top3 = &elves[len - 3..];
    top3.iter().sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 69836);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 207968);
    }
}