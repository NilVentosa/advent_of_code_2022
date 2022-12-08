use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const FILENAME: &str = "input.txt";

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn get_input() -> BufReader<File> {
    BufReader::new(File::open(FILENAME).expect("Error opening file"))
}

fn part_one() -> i32 {
    let mut total = 0;

    let folder_sizes = get_folder_sizes();
    for key in folder_sizes.keys() {
        let size = folder_sizes.get(key).unwrap();
        if &size <= &&100000 {
            total += size;
        }
    }

    return total;
}

fn get_all_folders() -> HashMap<String, i32> {
    let mut folders: HashMap<String, i32> = HashMap::new();

    let lines = get_input().lines();

    for line in lines {
        let line = line.unwrap();

        if line.starts_with("dir") || line.starts_with("$ cd ") && !line.starts_with("$ cd ..") {
            folders.insert(line.split(' ').last().unwrap().to_string(), 0);
        }
    }

    folders
}

fn get_folder_sizes() -> HashMap<String, i32> {
    let lines = get_input().lines();
    let mut folders = get_all_folders();

    let mut route: Vec<String> = vec![];

    for line in lines {
        let line = line.unwrap();

        if line.eq("$ cd ..") {
            route.pop();
            continue;
        }
        if line.starts_with("$ cd ") {
            route.push(line[5..].to_string());
            continue;
        }
        if !line.starts_with("$") && !line.starts_with("dir ") {
            for dir in &route {
                let current_value = folders.get(&dir as &str).unwrap();
                let parsed_value = &line.split(' ').collect::<Vec<&str>>()[0]
                    .parse::<i32>()
                    .unwrap();
                folders.insert(dir.to_string(), current_value + parsed_value);

            }
        }
    }
    folders
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
