use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::min;

// Not my proudest moment, but it works
// ETA: 2.5 hours on my machine LMAO
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let seeds = lines.next().unwrap().unwrap();
    let seed_nums : Vec<u64> = seeds.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|x| x.len() != 0)
        .map(|x|
            u64::from_str_radix(x, 10).unwrap())
        .collect();

    let mut map_idx = 0;
    let mut maps : Vec<Vec<Vec<u64>>> = vec![ vec![]; 7];
    for line in lines {
        let line_str = line.unwrap();
        if line_str.is_empty() {
            continue;
        }
        if line_str.contains("map") {
            map_idx += 1;
            continue;
        }
        let nums = line_str.split(' ').map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
        maps[map_idx - 1].push(nums);
    }

    let mut minimum_location = u64::MAX;   
    for (i, seed_start) in seed_nums.clone().into_iter().enumerate() {
        if i % 2 != 0 {
            continue;
        }
        for mut seed in seed_start..(seed_start + seed_nums[i + 1]) {
            for map in &maps {
                for entry in map {
                    if seed >= entry[1] && seed < entry[1] + entry[2] {
                        seed = seed - entry[1] + entry[0];
                        break;
                    }
                }
            }
            minimum_location = min(minimum_location, seed);
        }
    }

    println!("Minimum Location: {}", minimum_location);
}
