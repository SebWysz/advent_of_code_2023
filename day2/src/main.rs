use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use core::cmp::max;

fn main() {
    let file = File::open("input.txt").expect("File not found");

    let reader = BufReader::new(file);
    

    let mut sum_power = 0;

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let games = line.split(":").collect::<Vec<&str>>();

        let mut map : HashMap<&str, i32> = HashMap::new();

        for game in games[1].split(";") {
            let color_vec : Vec<&str> = game.split(",").collect();
            for color in color_vec {
                let entry : Vec<&str> = color.split(" ").collect();
                let curr_map_val = map.entry(entry[2]).or_insert(0).clone();

                map.insert(entry[2], max(curr_map_val, entry[1].parse::<i32>().unwrap()));
            }
        }
        
        sum_power += map["red"] * map["green"] * map["blue"];
    }

    println!("Result: {}", sum_power);
}
