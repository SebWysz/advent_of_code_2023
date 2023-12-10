use std::fs::File;
use std::io::{BufRead, BufReader};


struct StringToDigit {
    string : String,
    digit  : i32
}

impl StringToDigit {
    fn new(s : String, d : i32) -> StringToDigit {
        StringToDigit {
            string: s,
            digit: d
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Error Opening File");
    let reader = BufReader::new(file);
    
    let mut sum = 0;

    let lookup = vec![
        StringToDigit::new(String::from("one"), 1),
        StringToDigit::new(String::from("two"), 2),
        StringToDigit::new(String::from("three"), 3),
        StringToDigit::new(String::from("four"), 4),
        StringToDigit::new(String::from("five"), 5),
        StringToDigit::new(String::from("six"), 6),
        StringToDigit::new(String::from("seven"), 7),
        StringToDigit::new(String::from("eight"), 8),
        StringToDigit::new(String::from("nine"), 9),

    ];


    for line in reader.lines() {
        if line.is_err() {
            break;
        }
        let line = line.unwrap();

        let mut first : i32 = -1;
        let mut last  : i32 = -1;

        let mut first_idx = usize::MAX; // Possible error
        let mut last_idx = 0;
        
        for (num, char) in line.chars().enumerate() {
            
            if !char.is_ascii_digit() {
                continue;
            }
            let digit = char.to_digit(10).unwrap() as i32;
            last = digit;
            last_idx = num;

            if first == -1 {
                first = digit;
                first_idx = num;
            }
        }

        for lk in &lookup {
            for (idx, _) in line.match_indices(&lk.string) {
                if idx < first_idx {
                    first = lk.digit;
                    first_idx = idx;
                }
                if idx > last_idx {
                    last = lk.digit;
                    last_idx = idx;
                }
            }
        }

        sum += first * 10 + last;

    }
    println!("Result: {}", sum);
}
