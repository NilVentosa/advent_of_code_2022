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
    let lines = get_input("input.txt").lines();

    for line in lines {
        let line = line.unwrap();
        let (one, two) = get_assignments(&line);
        let one_limits = get_limits(one);
        let two_limits = get_limits(two);

        if contains(one_limits, two_limits) {
            result += 1;
        }
    }
    result
}

fn contains(left: (i32, i32), right: (i32, i32)) -> bool {
    let (left_a, right_a) = left;
    let (left_b, right_b) = right;

    if left_a <= left_b && right_a >= right_b {
        return true;
    }
    if left_b <= left_a && right_b >= right_a {
        return true;
    }
    false
}

fn overlaps(left: (i32, i32), right: (i32, i32)) -> bool {
    let (left_a, right_a) = left;
    let (left_b, right_b) = right;

    if right_a < left_b || right_b < left_a {
        return false;
    }
    true
}

fn get_limits(assignment: &str) -> (i32, i32) {
    let thing: Vec<&str> = assignment.split('-').collect();
    let one: i32 = thing.get(0).unwrap().parse().unwrap();
    let two: i32 = thing.get(1).unwrap().parse().unwrap();
    (one, two)
}

fn get_assignments(line: &str) -> (&str, &str) {
    let thing: Vec<&str> = line.split(',').collect();
    let one = thing.get(0).unwrap();
    let two = thing.get(1).unwrap();
    (one, two)
}

fn part_two() -> i32 {
    let mut result = 0;
    let lines = get_input("input.txt").lines();

    for line in lines {
        let line = line.unwrap();
        let (one, two) = get_assignments(&line);
        let one_limits = get_limits(one);
        let two_limits = get_limits(two);

        if overlaps(one_limits, two_limits) {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 424);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_contains() {
        let result = super::contains((2, 8), (3, 7));
        assert_eq!(result, true);
    }

    #[test]
    fn test_not_contains() {
        let result = super::contains((6, 10), (4, 6));
        assert_eq!(result, false);
    }

    #[test]
    fn test_get_assignments() {
        let result = super::get_assignments("2-5,3-4");
        assert_eq!(result, ("2-5", "3-4"));
    }
}
