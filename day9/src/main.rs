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

fn parse_input() -> Vec<Movement> {
    let mut parsed: Vec<Movement> = Vec::new();
    for line in get_input().lines() {
        let line = line.unwrap();
        let mut splitted = line.split_whitespace().into_iter();
        let direction = splitted.next().unwrap();
        let amount = splitted.next().unwrap();
        if direction == "R" {
            parsed.push(Movement::R(amount.parse::<i32>().unwrap()));
        } else if direction == "L" {
            parsed.push(Movement::L(amount.parse::<i32>().unwrap()));
        } else if direction == "U" {
            parsed.push(Movement::U(amount.parse::<i32>().unwrap()));
        } else if direction == "D" {
            parsed.push(Movement::D(amount.parse::<i32>().unwrap()));
        }
    }
    parsed
}

#[derive(Debug)]
enum Movement {
    R(i32),
    L(i32),
    D(i32),
    U(i32),
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
    fn is_touching(&self, b: &Point) -> bool {
        for i in &[-1, 0, 1] {
            for j in &[-1, 0, 1] {
                let new_tail = Point::new(&b.x + i, &b.y + j);
                if self.x == new_tail.x && self.y == new_tail.y {
                    return true;
                }
            }
        }
        false
    }

    fn next(&self, movement: &Movement) -> Point {
        match movement {
            Movement::R(_) => return Point::new(self.x + 1, self.y),
            Movement::L(_) => return Point::new(self.x - 1, self.y),
            Movement::U(_) => return Point::new(self.x, self.y + 1),
            Movement::D(_) => return Point::new(self.x, self.y - 1),
        };
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Snake {
    size: usize,
    visited: HashSet<Point>,
    points: Vec<Point>,
}

impl Snake {
    fn new(size: usize, start: Point) -> Snake {
        let mut snake = Snake {
            size,
            visited: HashSet::new(),
            points: Vec::new(),
        };
        snake.visited.insert(start);
        snake.points.push(start);
        snake
    }

    fn do_movement(&mut self, movement: Movement) {
        let amount: i32;
        match movement {
            Movement::R(i) => amount = i,
            Movement::L(i) => amount = i,
            Movement::U(i) => amount = i,
            Movement::D(i) => amount = i,
        }

        for _ in 0..amount {
            // Always add new head
            let next = self.points.get(0).unwrap().next(&movement);
            self.points.insert(0, next);

            // When changing directions remove corner
            if self.points.len() > 2 && next.is_touching(self.points.get(2).unwrap()) {
                self.points.remove(1);
            }

            // When head not touching move rest
            else if self.points.len() > 2 && !next.is_touching(self.points.get(2).unwrap()) {
                self.points.remove(1);
            }
            

            // Remove from the tail if longer than size
            if self.points.len() > self.size {
                self.points.remove(self.points.len() - 1);
            }
            // add tail to visited set
            self.visited.insert(self.points.last().unwrap().clone());

            //     self.points.insert(0, next);
            //     if self.points.len() < 3 {
            //         self.visited.insert(self.points.last().unwrap().clone());
            //         continue;
            //     }
            //     // When head changes direction remove prev head
            //     if self
            //         .points
            //         .get(2)
            //         .unwrap()
            //         .is_touching(self.points.get(0).unwrap())
            //     {
            //         self.points.remove(1);
            //         continue;
            //     }
            //     let diff_x = next.x - self.points.get(2).unwrap().x;
            //     let diff_y = next.y - self.points.get(2).unwrap().y;
            //
            //     // When head detaches
            //     if !self
            //         .points
            //         .get(2)
            //         .unwrap()
            //         .is_touching(self.points.get(0).unwrap())
            //     {
            //         for i in 0..self.points.len()-3 {
            //             if !self.points.get(i).unwrap().is_touching(self.points.get(i+2).unwrap()) {
            //                 self.points.remove(i+1);
            //                 self.points.get_mut(1).unwrap().x += diff_x;
            //                 self.points.get_mut(1).unwrap().y += diff_y;
            //             }
            //         }
            //     }
            //     if self.points.len() < self.size {
            //         self.points.push(Point::new(0, 0));
            //     }
            //     if self.points.len() > self.size {
            //         self.points.remove(self.points.len() - 1);
            //     }
            //     self.visited.insert(self.points.last().unwrap().clone());
        }
    }
}

fn part_one() -> usize {
    let parsed = parse_input();
    let mut snake = Snake::new(2, Point::new(0, 0));

    for item in parsed {
        snake.do_movement(item);
    }
    return snake.visited.len();
}

fn part_two() -> usize {
    let parsed = parse_input();
    let mut snake = Snake::new(10, Point::new(0, 0));

    for item in parsed {
        snake.do_movement(item);
    }
    return snake.visited.len();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, 5902);
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, 36);
    }

    #[test]
    fn test_are_touching() {
        let head = super::Point::new(1, 0);
        let tail = super::Point::new(0, 0);
        assert_eq!(&true, &head.is_touching(&tail));
    }
}
