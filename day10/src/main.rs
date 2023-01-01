use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const FILENAME: &str = "test_input.txt";

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn get_input() -> BufReader<File> {
    BufReader::new(File::open(FILENAME).expect("Error opening file"))
}

fn part_one() -> isize {
    let lines = get_input().lines();

    let cycles: Vec<isize> = vec![20, 60, 100, 140, 180, 220];
    let mut current_cycle = 1;
    let mut x = 1;
    let mut result: isize = 0;

    for line in lines {
        let line = line.unwrap();
        let thing: Vec<&str> = line.split_whitespace().into_iter().collect();
        if thing.len() == 2 {
            current_cycle += 2;
            x += thing.get(1).unwrap().parse::<isize>().unwrap();
        } else {
            current_cycle += 1;
        }
    }

    x
}

fn part_two() -> isize {
    let lines = get_input().lines();

    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 13140);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 1);
    }
}
