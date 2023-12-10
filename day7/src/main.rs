use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut hands = Vec::new();
    let mut winnings = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let card_bid = line.split(" ").collect::<Vec<&str>>();

        let cards = card_bid[0].chars().collect::<Vec<char>>();
        let bid = card_bid[1].parse::<u64>().unwrap();

        hands.push(Hand::new(cards, bid));
    }
    hands.sort();
    for (idx, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (idx + 1) as u64;
    }

    println!("Result: {}", winnings);
}

struct Hand {
    cards: Vec<char>,
    value: u64,
    bid: u64,
}

impl Hand {
    fn new(cards : Vec<char>, bid : u64) -> Hand {
        let mut hand = Hand {
            cards,
            value: 0,
            bid,
        };
        hand.calculate_value();
        return hand;
    }

    fn calculate_value(&mut self) {
        let mut lookup = vec![0; 12];
        let mut joker = 0;
        let mut value = 0;
        for card in &self.cards {
            match card {
                'A' => lookup[0] += 1,
                'K' => lookup[1] += 1,
                'Q' => lookup[2] += 1,
                'T' => lookup[3] += 1,
                '9' => lookup[4] += 1,
                '8' => lookup[5] += 1,
                '7' => lookup[6] += 1,
                '6' => lookup[7] += 1,
                '5' => lookup[8] += 1,
                '4' => lookup[9] += 1,
                '3' => lookup[10] += 1,
                '2' => lookup[11] += 1,
                'J' => joker += 1,
                _ => (),
            }
        }
        lookup.sort();

        for card_count in lookup.iter().rev() {
            value += 10u64.pow((card_count + joker) as u32);
            joker = 0;
        }
        self.value = value;
    }

}

fn card_value(card : char) -> u64 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!("Char is not a valid card"),
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value 
        && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.value > other.value {
            cmp::Ordering::Greater
        } else if self.value < other.value {
            cmp::Ordering::Less
        } else {
            if self.cards == other.cards {
                return cmp::Ordering::Equal;
            }
            for idx in 0..5 {
                if self.cards[idx] != other.cards[idx] {
                    if card_value(self.cards[idx]) > card_value(other.cards[idx]) {
                        return cmp::Ordering::Greater;
                    }
                    return cmp::Ordering::Less;
                }
            }
            return cmp::Ordering::Equal
        }
    }
}