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

struct Grid {
    rows: Vec<Row>,
}

struct Row {
    trees: Vec<usize>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<usize> {
        self.rows.get(y).unwrap().trees.get(x).copied()
    }

    fn get_score(&self, x: usize, y: usize) -> usize {
        let hight = self.get(x, y).unwrap();
        let mut score = 1;

        //left
        let mut result = 0;
        let mut i = x;
        while i > 0 {
            i -= 1;
            match self.get(i, y) {
                None => return result,
                Some(tree) => {
                    if tree < hight {
                        result += 1;
                    } else {
                        result += 1;
                        break;
                    }
                }
            }
        }
        score *= result;
        //right
        let mut result = 0;
        for i in x + 1..self.rows.len() {
            match self.get(i, y) {
                None => return result,
                Some(tree) => {
                    if tree < hight {
                        result += 1;
                    } else {
                        result += 1;
                        break;
                    }
                }
            }
        }
        score *= result;
        //up
        let mut result = 0;
        let mut i = y;
        while i > 0 {
            i -= 1;
            match self.get(x, i) {
                None => return result,
                Some(tree) => {
                    if tree < hight {
                        result += 1;
                    } else {
                        result += 1;
                        break;
                    }
                }
            }
        }
        score *= result;
        //down
        let mut result = 0;
        for i in y + 1..self.rows.get(0).unwrap().trees.len() {
            match self.get(x, i) {
                None => return result,
                Some(tree) => {
                    if tree < hight {
                        result += 1;
                    } else {
                        result += 1;
                        break;
                    }
                }
            }
        }
        score *= result;

        score
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let hight = self.get(x, y).unwrap();

        //left
        let mut result = true;
        for i in 0..x {
            if i == x {
                return true;
            }
            match self.get(i, y) {
                None => return result,
                Some(tree) => {
                    if tree >= hight {
                        result = false;
                    }
                }
            }
        }
        if result == true {
            return result;
        }

        //right
        let mut result = true;
        for i in x + 1..self.rows.len() {
            match self.get(i, y) {
                None => return result,
                Some(tree) => {
                    if tree >= hight {
                        result = false;
                    }
                }
            }
        }
        if result == true {
            return result;
        }

        //up
        let mut result = true;
        for i in 0..y {
            if i == y {
                return true;
            }
            match self.get(x, i) {
                None => return result,
                Some(tree) => {
                    if tree >= hight {
                        result = false;
                    }
                }
            }
        }
        if result == true {
            return result;
        }

        //down
        let mut result = true;
        for i in y + 1..self.rows.get(0).unwrap().trees.len() {
            match self.get(x, i) {
                None => return result,
                Some(tree) => {
                    if tree >= hight {
                        result = false;
                    }
                }
            }
        }
        result
    }
}

fn part_one() -> usize {
    let mut result = 0;
    let lines = get_input().lines();

    let mut grid = Grid { rows: Vec::new() };

    for line in lines {
        let line = line.unwrap();
        let mut row = Row { trees: Vec::new() };
        for c in line.chars() {
            row.trees.push(c.to_string().parse::<usize>().unwrap());
        }
        grid.rows.push(row);
    }

    for i in 0..grid.rows.len() {
        for j in 0..grid.rows.get(0).unwrap().trees.len() {
            if grid.is_visible(j, i) {
                result += 1;
            }
        }
    }

    return result;
}

fn part_two() -> usize {
    let mut result = 0;
    let lines = get_input().lines();

    let mut grid = Grid { rows: Vec::new() };

    for line in lines {
        let line = line.unwrap();
        let mut row = Row { trees: Vec::new() };
        for c in line.chars() {
            row.trees.push(c.to_string().parse::<usize>().unwrap());
        }
        grid.rows.push(row);
    }

    for i in 0..grid.rows.len() {
        for j in 0..grid.rows.get(0).unwrap().trees.len() {
            let score = grid.get_score(j, i);
            if score > result {
                result = score;
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 1801);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 8);
    }
}
