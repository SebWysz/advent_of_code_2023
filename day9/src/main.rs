use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let nums = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        sum += extrapolate_next(&nums);
    }

    println!("Result: {}", sum);
}

fn extrapolate_next(nums: &Vec<i64>) -> i64 {
    if nums.iter().all(|&x| x == 0) {
        return 0;
    }
    let mut differences = vec![];
    for i in 0..nums.len() - 1 {
        differences.push(nums[i + 1] - nums[i]);
    }
    
    let diff = extrapolate_next(&differences);
    return nums.first().unwrap() - diff;    
}
