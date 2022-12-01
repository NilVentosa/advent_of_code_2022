use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    let mut current = 0;

    let br = BufReader::new(File::open("input.txt").unwrap());

    for line in br.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if current > one {
                three = two;
                two = one;
                one = current;
            } else if current > two {
                three = two;
                two = current;
            } else if current > three {
                three = current;
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("{}", one + two + three);
}

fn part_one() {
    let mut largest = 0;
    let mut current = 0;

    let br = BufReader::new(File::open("input.txt").unwrap());

    for line in br.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if current > largest {
                largest = current;
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("{}", largest);
}
