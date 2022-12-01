use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn open(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).unwrap())
}

fn get_sorted_entries(br: BufReader<File>) -> Vec<i32> {
    let mut current = 0;
    let mut entries = vec![];

    for line in br.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            entries.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    entries.sort();
    entries
}

fn part_two() -> i32 {
    let br = open("input.txt");
    let sorted = get_sorted_entries(br);

    sorted.get(sorted.len() - 1).unwrap()
        + sorted.get(sorted.len() - 2).unwrap()
        + sorted.get(sorted.len() - 3).unwrap()
}

fn part_one() -> i32 {
    let br = open("input.txt");
    let sorted = get_sorted_entries(br);
    *sorted.last().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 71924);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 210406);
    }
}
