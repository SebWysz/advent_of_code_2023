use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut board = vec![];
    let mut curr_xy = (0, 0);

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let mut row = vec![];
        line.chars().for_each(|c| {
            if c == 'S' {
                curr_xy = (row.len(), board.len());
            }
            row.push(c);
        });
        board.push(row);
    });
    let mut cycle: HashSet<(usize, usize)> = HashSet::new();
    let _steps = follow_directions(&board, curr_xy, &mut cycle) / 2;
    let rays = create_rays(&board, &cycle);
    let mut num_tiles = 0;
    for (y, vec) in board.iter().enumerate() {
        for (x, ch) in vec.iter().enumerate() {
            if cycle.contains(&(x, y)) {
                continue;
            }
            if count_rays(&rays, x, y) % 2 != 0 {
                // println!("Count : {}, char : {}, Tile: {}, {}", count_rays(&rays, x, y), *ch, x, y);
                num_tiles += 1;
            }

            
        }
    }
    
    

    println!("Result: {}", num_tiles);
}

fn start_dir(board: &Vec<Vec<char>>, curr_xy: (usize, usize)) -> (usize, usize) {
    let (x, y) = curr_xy;
    let mut dir = (0, 0);
    if x > 0 && left_chars(board[y][x-1]) {
        dir = (x - 1, y);
    } else if x < board[y].len() - 1 && right_chars(board[y][x+1]) {
        dir = (x + 1, y);
    } else if y > 0 && up_chars(board[y-1][x]) {
        dir = (x, y - 1);
    } else if y < board.len() - 1 && down_chars(board[y+1][x]) {
        dir = (x, y + 1);
    }
    dir
}

fn follow_directions(board: &Vec<Vec<char>>, curr_xy: (usize, usize), hash_set: &mut HashSet<(usize, usize)>) -> usize {
    let (mut x, mut y) = curr_xy;
    let mut steps: usize = 0;
    let mut came_from = (x, y);
    hash_set.insert((x, y));
    (x, y) = start_dir(board, curr_xy);
    steps += 1;
    while board[y][x] != 'S' {
        hash_set.insert((x, y));
        match board[y][x] {
            'L' => {
                if came_from.0 + 1 - x == 2 {
                    came_from = (x, y);
                    (x, y) = (x, y - 1);
                } else {
                    came_from = (x, y);
                    (x, y) = (x + 1, y);
                }
            }
            '7' => {
                if x + 1 - came_from.0 == 2 {
                    came_from = (x, y);
                    (x, y) = (x, y + 1);
                } else {
                    came_from = (x, y);
                    (x, y) = (x - 1, y);
                }
            }
            'F' => {
                if came_from.0  + 1 - x == 2 {
                    came_from = (x, y);
                    (x, y) = (x, y + 1);
                } else {
                    came_from = (x, y);
                    (x, y) = (x + 1, y);
                }
            }
            'J' => {
                if x + 1 - came_from.0 == 2 {
                    came_from = (x, y);
                    (x, y) = (x, y - 1);
                } else {
                    came_from = (x, y);
                    (x, y) = (x - 1, y);
                }
            }
            '-' => {
                if came_from.0 + 1 - x == 2 {
                    came_from = (x, y);
                    (x, y) = (x - 1, y);
                } else {
                    came_from = (x, y);
                    (x, y) = (x + 1, y);
                }
            }
            '|' => {
                if came_from.1 + 1 - y == 2 {
                    came_from = (x, y);
                    (x, y) = (x, y - 1);
                } else {
                    came_from = (x, y);
                    (x, y) = (x, y + 1);
                }
            }
            c => panic!("Invalid character {}, tile ({}, {}) CameFrom char {}, tile: ({}, {})", c, x, y, board[came_from.0][came_from.1], came_from.0, came_from.1),
        }
        steps += 1;
    }
    steps
}

fn left_chars(ch : char) -> bool {
    ch == '-' || ch == 'L' || ch == 'F'
}

fn right_chars(ch : char) -> bool {
    ch == '-' || ch == 'J' || ch == '7'
}

fn down_chars(ch : char) -> bool {
    ch == '|' || ch == 'J' || ch == 'L'
}

fn up_chars(ch : char) -> bool {
    ch == '|' || ch == 'F' || ch == '7'
}

fn create_rays(board: &Vec<Vec<char>>, cycle : &HashSet<(usize, usize)>) -> Vec<Vec<bool>> {
    let mut rays: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];

    for (y, vec) in board.iter().enumerate() {
        for (x, _) in vec.iter().enumerate() {
            if cycle.contains(&(x, y)) && (up_chars(board[y][x]) || board[y][x] == 'S') && y + 1 < board.len() && (down_chars(board[y+1][x]) || board[y+1][x] == 'S') {
                rays[y][x] = true;
            }
        }
    }

    rays
}

fn count_rays(rays: &Vec<Vec<bool>>, x : usize, y : usize) -> usize {
    let mut count = 0;
    for i in 0..x {
        if rays[y][i] {
            count += 1;
        }
    }
    count
}