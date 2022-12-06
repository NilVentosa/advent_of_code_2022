use std::collections::HashSet;
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

fn part_one() -> usize {
    return find_marker(4).unwrap();
}

fn part_two() -> usize {
    return find_marker(14).unwrap();
}

fn find_marker(length: usize) -> Option<usize> {
    let text = get_input("input.txt").lines().next().unwrap().unwrap();

    for i in 0..text.len() - length {
        let set: HashSet<char> = text[i..i + length].chars().collect();
        if set.len() == length {
            return Some(i + length);
        }
    }
    return None;

}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 1140);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 3495);
    }
}
