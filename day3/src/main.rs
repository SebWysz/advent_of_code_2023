use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut array = vec![];

    for line in reader.lines() {
        let chars : Vec<char> = line.unwrap().chars().collect();
        array.push(chars);
    }

    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    for i in 0..array.len() {
        let mut skips = 0;

        for j in 0..array[0].len() {
            if skips != 0 {
                skips -= 1;
                continue;
            }

            if !array[i][j].is_digit(10) {
                continue;
            }
            let ret_val = find_symbol(&array, i as i32, j as i32);
            if ret_val.0.is_none() {
                continue;
            }

            let num_width = ret_val.1;

            let mut sum = 0;
            for k in 0..num_width {
                
                let res = array[i][j + k as usize].to_digit(10).unwrap() * u32::pow(10, (num_width - k - 1).try_into().unwrap() );
                sum += res;
            }
            let curr_map_val = map.entry(ret_val.0.unwrap()).or_insert(vec![]); //.unwrap_or(&vec![]);
            curr_map_val.push(sum as i32);
            // map.insert(&ret_val.0.unwrap(), curr_map_val);
            skips = num_width - 1;
        }
    }
    let mut final_sum = 0;
    for (_, values) in map {
        if values.len() == 1 {
            continue;
        }
        let mut mult = 1;
        for value in values {
            mult *= value;
        }
        final_sum += mult;
    }

    println!("Result: {}", final_sum);
}


fn find_symbol(array : &Vec<Vec<char>>, i : i32, j : i32) -> (Option<(i32, i32)>, i32) {

    let directions = [
        (1, 0), (-1, 0), (0, -1), (0, 1),
        (1, 1), (-1, 1), (1, -1), (-1, -1)
    ];

    let mut location = (-1, -1);
    let mut num_width = 1;

    for (dir_i, dir_j) in directions {
        let new_i = i + dir_i;
        let new_j = j + dir_j;

        if new_i >= array.len() as i32 || new_i < 0
        || new_j >= array[0].len() as i32 || new_j < 0 {
            continue;
        }

        let curr_char = array[new_i as usize][new_j as usize];

        if new_i == i && new_j > j && curr_char.is_digit(10) {
            let ret_val = find_symbol(array, new_i, new_j);
            num_width += ret_val.1;

            if ret_val.0.is_some() {
                location = ret_val.0.unwrap();
            }
            
        } else if curr_char == '*' {
            location = (new_i, new_j);
        }
    }

    if location != (-1, -1) {
        return (Some(location), num_width);
    }
    return (None, num_width);
}