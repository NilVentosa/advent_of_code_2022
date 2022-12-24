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

fn part_one() -> usize {
    let lines = get_input().lines();

    return 0;
}

fn part_two() -> usize {
    let lines = get_input().lines();

    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 1);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 1);
    }
}
