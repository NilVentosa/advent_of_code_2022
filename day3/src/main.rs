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
        result += char_to_value(get_common_item_in_line(&line.unwrap()).unwrap());
    }
    result
}

fn part_two() -> i32 {
    let mut result = 0;
    let mut one: String;
    let mut two: String;
    let mut three: String;

    let mut lines = get_input("input.txt").lines().peekable();
    while lines.peek().is_some() {
        one = lines.next().unwrap().unwrap();
        two = lines.next().unwrap().unwrap();
        three = lines.next().unwrap().unwrap();

        result += char_to_value(get_common_item_in_three_lines((&one, &two, &three)).unwrap());
    }

    result
}

fn get_common_item_in_line(ruckstack: &str) -> Option<char> {
    let (first, second) = ruckstack.split_at(ruckstack.len() / 2);
    for c in first.chars() {
        if second.contains(c) {
            return Some(c);
        }
    }
    None
}

fn get_common_item_in_three_lines((one, two, three): (&str, &str, &str)) -> Option<char> {
    for c in one.chars() {
        if two.contains(c) && three.contains(c) {
            return Some(c);
        }
    }
    None
}

fn char_to_value(c: char) -> i32 {
    if c.is_uppercase() {
        return c as i32 - 38;
    } else {
        return c as i32 - 96;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 7831);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 2683);
    }

    #[test]
    fn test_get_common_item() {
        let result = super::get_common_item_in_line("aAsEsR");
        assert_eq!(result.unwrap(), 's');
    }

    #[test]
    fn test_char_to_value() {
        assert_eq!(super::char_to_value('B'), 28);
        assert_eq!(super::char_to_value('c'), 3);
    }
}
