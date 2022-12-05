use std::collections::HashMap;
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

fn part_one() -> String {
    let mut result = "".to_string();

    let mut stacks = get_stacks(get_top());
    let instructions = get_instructions();

    for instruction in instructions {
        stacks = instruction.run_one(stacks);
    }

    for i in 1..stacks.len() + 1 {
        result.push(stacks.get(&i).unwrap().last().unwrap().parse().unwrap());
    }

    return result;
}

fn part_two() -> String {
    let mut result = "".to_string();

    let mut stacks = get_stacks(get_top());
    let instructions = get_instructions();

    for instruction in instructions {
        stacks = instruction.run_two(stacks);
    }

    for i in 1..stacks.len() + 1 {
        result.push(stacks.get(&i).unwrap().last().unwrap().parse().unwrap());
    }

    return result;
}

#[derive(Debug)]
struct Instruction {
    moove: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from(line: String) -> Instruction {
        let splitted: Vec<&str> = line.split(' ').collect();
        Instruction {
            moove: splitted.get(1).unwrap().parse().unwrap(),
            from: splitted.get(3).unwrap().parse().unwrap(),
            to: splitted.get(5).unwrap().parse().unwrap(),
        }
    }

    fn run_one(&self, mut stacks: HashMap<usize, Vec<String>>) -> HashMap<usize, Vec<String>> {
        for _i in 0..self.moove {
            let c = stacks.get_mut(&self.from).unwrap().pop();
            stacks.get_mut(&self.to).unwrap().push(c.unwrap());
        }

        stacks
    }

    fn run_two(&self, mut stacks: HashMap<usize, Vec<String>>) -> HashMap<usize, Vec<String>> {
        let mut temp: Vec<String> = Vec::new();
        for _i in 0..self.moove {
            temp.push(stacks.get_mut(&self.from).unwrap().pop().unwrap());
        }

        while !temp.is_empty() {
            stacks.get_mut(&self.to).unwrap().push(temp.pop().unwrap());
        }

        stacks
    }
}

fn get_stacks(top: Vec<String>) -> HashMap<usize, Vec<String>> {
    let mut parsed_top: Vec<Vec<String>> = Vec::new();

    for _c in top.get(0).unwrap().chars() {
        let v = Vec::new();
        parsed_top.push(v);
    }

    for line in top {
        let mut i = 0;
        for c in line.chars() {
            let thing: &mut Vec<String> = parsed_top.get_mut(i).unwrap();
            thing.push(c.to_string());
            i += 1;
        }
    }

    let mut clean_top_temp: Vec<Vec<String>> = Vec::new();

    for item in parsed_top {
        if item.get(0).unwrap().to_string() != " ".to_string() {
            clean_top_temp.push(item);
        }
    }

    let mut clean_top: Vec<Vec<String>> = Vec::new();

    for mut item in clean_top_temp {
        item.retain(|x| x != " ");
        clean_top.push(item);
    }

    let mut stacks: HashMap<usize, Vec<String>> = HashMap::new();

    for mut item in clean_top {
        stacks.insert(item.remove(0).parse().unwrap(), item);
    }

    stacks
}

fn get_top() -> Vec<String> {
    let lines = get_input("input.txt").lines();
    let mut top: Vec<String> = Vec::new();

    let mut thing = true;
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            thing = false;
            continue;
        }
        if thing {
            top.push(line.to_string());
            continue;
        }
    }
    let mut reverse_top: Vec<String> = Vec::new();

    for item in top.iter().rev() {
        reverse_top.push(item.to_string());
    }
    reverse_top
}

fn get_instructions() -> Vec<Instruction> {
    let lines = get_input("input.txt").lines();
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut thing = true;
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            thing = false;
            continue;
        }
        if !thing {
            instructions.push(Instruction::from(line.to_string()));
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let result = super::part_one();
        assert_eq!(result, "JRVNHHCSJ");
    }

    #[test]
    fn part_two() {
        let result = super::part_two();
        assert_eq!(result, "GNFBSBJLH");
    }
}
