use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn get_input(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).expect("Error opening file"))
}

fn part_one() -> i32 {
    let mut result = 0;

    for line in get_input("input.txt").lines() {
        let line = line.unwrap();

        let oponent = line.chars().next().unwrap();
        let you = line.chars().nth(2).unwrap();
        result += evaluate_line_one(&oponent, &you);
    }

    result
}

fn part_two() -> i32 {
    let mut result = 0;

    for line in get_input("input.txt").lines() {
        let line = line.unwrap();

        let oponent = line.chars().next().unwrap();
        let you = line.chars().nth(2).unwrap();
        result += evaluate_line_two(&oponent, &you);
    }

    result
}

fn evaluate_line_two(oponent: &char, you: &char) -> i32 {
    let result;

    if oponent == &'A' && you == &'X' {
        result = 3;
    } else if oponent == &'A' && you == &'Y' {
        result = 4;
    } else if oponent == &'A' && you == &'Z' {
        result = 8;
    } else if oponent == &'B' && you == &'X' {
        result = 1;
    } else if oponent == &'B' && you == &'Y' {
        result = 5;
    } else if oponent == &'B' && you == &'Z' {
        result = 9;
    } else if oponent == &'C' && you == &'X' {
        result = 2;
    } else if oponent == &'C' && you == &'Y' {
        result = 6;
    } else {
        result = 7;
    }
    return result;
}

fn evaluate_line_one(oponent: &char, you: &char) -> i32 {
    let result;

    if oponent == &'A' && you == &'X' {
        result = 4;
    } else if oponent == &'A' && you == &'Y' {
        result = 8;
    } else if oponent == &'A' && you == &'Z' {
        result = 3;
    } else if oponent == &'B' && you == &'X' {
        result = 1;
    } else if oponent == &'B' && you == &'Y' {
        result = 5;
    } else if oponent == &'B' && you == &'Z' {
        result = 9;
    } else if oponent == &'C' && you == &'X' {
        result = 7;
    } else if oponent == &'C' && you == &'Y' {
        result = 2;
    } else {
        result = 6;
    }
    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 10404);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 10334);
    }
}
