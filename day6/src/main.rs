use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file).lines();

    let time_str_spaces = reader.next().unwrap().unwrap();
    let distance_str_spaces= reader.next().unwrap().unwrap();

    let time_str = time_str_spaces.split(':')
                                .collect::<Vec<&str>>()[1]
                                .chars()
                                .filter(|x| x.is_digit(10))
                                .collect::<String>();

    let time = u64::from_str_radix(&time_str, 10).unwrap();


    let distance_str = distance_str_spaces.split(':')
                                .collect::<Vec<&str>>()[1]
                                .chars()
                                .filter(|x| x.is_digit(10))
                                .collect::<String>();
    
    let distance = u64::from_str_radix(&distance_str, 10).unwrap();
                                
        
    let half = time / 2;
    let mut combination_count = 0;
    if (time - half) * half > distance {
        combination_count += 1;
    }
    for i in 1..half {
        if (time - half + i)*(half - i) > distance {
            combination_count += 1;
        }
        if (time - half - i) * (half + i) > distance {
            combination_count += 1;
        } else if (time - half + i) * (half - i) <= distance {
            break;
        }
    }

    println!("Result: {}", combination_count);
}