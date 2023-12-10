use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Error opening File");
    let reader = BufReader::new(file);

    let mut sum = 0;

    let mut num_cards = vec![1; 209];
    let mut curr_idx = 0;

    for res in reader.lines() {
        let line = res.unwrap();
        let card_nums = line.split(":").collect::<Vec<&str>>();

        let nums = card_nums[1].split('|').collect::<Vec<&str>>();
        let winner_slice = nums[0].split(' ').collect::<Vec<&str>>();
        let possible_slice = nums[1].split(' ').collect::<Vec<&str>>();
        
        let mut winnerwinnerchickendinner = Vec::new();

        for slice in winner_slice {
            if slice.len() == 0 {
                continue;
            }
            let num = i32::from_str_radix(slice, 10).expect("No number?");
            winnerwinnerchickendinner.push(num);
        }

        let mut num_winners: i32 = 0;

        for slice in possible_slice {
            if slice.len() == 0 {
                continue;
            }
            let num = i32::from_str_radix(slice, 10).expect("No number?");
            if winnerwinnerchickendinner.contains(&num) {
                num_winners += 1;
            }
        }
        for _ in 0..num_cards[curr_idx] {
            for j in 1..(num_winners + 1) {
                if curr_idx + j as usize >= num_cards.len() {
                    break;
                }
                num_cards[curr_idx + j as usize] += 1;
            }
            sum += 1;
        }
        curr_idx += 1;
    }

    println!("Result: {}", sum);
}
