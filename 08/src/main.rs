use std::env;
use std::fs;

fn main() {
    let name = env::args().nth(1)
        .expect("an input file argument");
    let items = fs::read_to_string(name)
        .expect("able to open the input file");

    let items: Vec<Vec<u32>> = items.lines()
        .map(|ln| ln.as_bytes()
            .iter()
            .map(|&x| (x - b'0') as u32)
            .collect::<Vec<u32>>()
        ).collect();

    let cols = items[0].len();
    let rows = items.len();

    {
        let mut count = 0;

        for (y, ln) in items.iter().enumerate() {
            for (x, &it) in ln.iter().enumerate() {
                let horz_greater = |i: usize| ln[i] >= it;
                let vert_greater = |i: usize| items[i][x] >= it;
                if (x+1..cols).any(horz_greater) && (0..x).any(horz_greater)
                && (y+1..rows).any(vert_greater) && (0..y).any(vert_greater) {
                    count += 1;
                }
            }
        }
        
        println!("[PART ONE] {}", cols*rows - count);
    }

    let mut best_score = 0;
    for (y, ln) in items.iter().enumerate() {
        for (x, &it) in ln.iter().enumerate() {
            let scenic_score = count(&mut ln.into_iter().take(x).rev().cloned(), it)
                             * count(&mut ln.into_iter().skip(x+1).cloned(), it)
                             * count(&mut items.iter().map(|ln| ln[x]).take(y).rev(), it)
                             * count(&mut items.iter().map(|ln| ln[x]).skip(y+1), it);
            best_score = best_score.max(scenic_score);
        }
    }
    println!("[PART TWO] {}", best_score);
}

// I can't just take_while because I want top stop *after* the failing element not before
fn count(items: &mut dyn std::iter::Iterator<Item = u32>, it: u32) -> u32 {
    let mut score = 0;
    for i in items {
        score += 1;
        if i >= it { break; }
    }
    score
}
