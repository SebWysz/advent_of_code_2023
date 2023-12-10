use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(|l| l.unwrap());
    let instructions = lines.next().unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut start = vec![];
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let mut words = line.split("=");
        let key = words.next().unwrap().trim();

        if key.ends_with('A') {
            start.push(key.to_string());
        }

        let mut l_r = words.next().unwrap().trim().split(",");
        let l = l_r.next().unwrap().trim().trim_start_matches('(');
        let r = l_r.next().unwrap().trim().trim_end_matches(')');
        map.insert(key.to_string(), (l.to_string(), r.to_string()));
    }

    let steps = start.iter().map(|s| count_steps(&map, &s, &instructions)).reduce(|a, b| lcm(a, b)).unwrap();

    println!("Steps: {}", steps);
}

fn count_steps(map: &HashMap<String, (String, String)>, start: &String, instructions: &str) -> u64 {
    let mut start = start.clone();
    let mut steps: u64 = 0;
    while !start.ends_with("Z") {
        for instruction in instructions.chars() {
            let (l, r) = map.get(&start).unwrap();
            match instruction {
                'L' => start = l.to_string(),
                'R' => start = r.to_string(),
                    _  => panic!("Unknown instruction: {}", instruction),
            }
            steps += 1;
            if start.ends_with("Z") {
                break;
            }
        }
    }
    steps
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}