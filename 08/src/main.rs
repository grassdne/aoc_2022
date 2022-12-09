use std::env;
use std::fs;

fn main() {
    let name = env::args().nth(1)
        .expect("an input file argument");
    let items = fs::read_to_string(name)
        .expect("able to open the input file");

    let items: Vec<Vec<u32>> = items.lines()
        .map(|ln| ln.as_bytes().iter()
            .map(|&x| (x - b'0') as u32)
            .collect::<Vec<u32>>()
        ).collect();

    let cols = items[0].len();
    let rows = items.len();

    {
        let mut count = 0;
        let mut best_score = 0;

        for (y, ln) in items.iter().enumerate() {
            for (x, &it) in ln.iter().enumerate() {
                let horz_greater = |i: usize| ln[i] >= it;
                let vert_greater = |i: usize| items[i][x] >= it;

                if (x+1..cols).any(horz_greater) && (0..x).any(horz_greater)
                && (y+1..rows).any(vert_greater) && (0..y).any(vert_greater)) {
                    count += 1;
                }

                let scenic_score = ((x+1..cols).find(|&i| horz_greater(i)).unwrap_or(cols-1) - x)
                                 * ((y+1..rows).find(|&i| vert_greater(i)).unwrap_or(rows-1) - y)
                                 * (x - (0..x).rev().find(|&i| horz_greater(i)).unwrap_or(0))
                                 * (y - (0..y).rev().find(|&i| horz_greater(i)).unwrap_or(0));
                best_score = best_score.max(scenic_score);
            }
        }
        
        println!("[PART ONE] {}", cols*rows - count);
        println!("[PART TWO] {}", best_score);
    }
}
