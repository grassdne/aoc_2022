use std::{process, env, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
    Full,
    Empty,
}

const FIELD_WIDTH: usize = 1000;
const FIELD_HEIGHT: usize = 200;
const SAND_SPAWN: usize = 500;

type Field = [[Space; FIELD_WIDTH]; FIELD_HEIGHT];

fn parse_line(field: &mut Field, ln: &str) {
    let vectors = ln.split(" -> ")
                    .map(|s| s.split_once(",")
                              .map(|(x,y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
                              .unwrap());

    for ((x1, y1), (x2, y2)) in vectors.clone().zip(vectors.clone().skip(1)) {
        for x in x1.min(x2) ..= x1.max(x2) {
            for y in y1.min(y2) ..= y1.max(y2) {
                field[y as usize][x as usize] = Space::Full;
            }
        }
    }
}

fn put_sand(field: &mut Field, x: usize, y: usize, floor: usize) -> bool {
    use Space::*;
    let row = field[y+1];
    // We want to panic on out of bounds
    match (row[x-1], row[x], row[x+1]) {
        (_, Empty, _) => put_sand(field, x, y+1, floor),
        (Empty, Full, _) => put_sand(field, x-1, y+1, floor),
        (Full, Full, Empty) => put_sand(field, x+1, y+1, floor),
        (Full, Full, Full) => {
            field[y][x] = Full;
            let at_bottom = y+1 >= floor;
            !at_bottom
        },
    }
}

fn main() {
    let name = env::args().skip(1).next().unwrap_or_else(|| {
        eprintln!("missing input file argument");
        process::exit(1);
    });
    let content = fs::read_to_string(&name).unwrap_or_else(|err| {
        eprintln!("failed reading file ({name}): {}", err);
        process::exit(1);
    });
    let mut field: Field = [[Space::Empty; FIELD_WIDTH]; FIELD_HEIGHT];
    content.lines().for_each(|ln| parse_line(&mut field, ln));

    // gods just use a fucking for loop
    let floor = field.iter().enumerate()
        .rfind(|(_, row)| row.iter().find(|x| **x == Space::Full).is_some())
        .map(|(i, _)| i+2).unwrap();

    for it in field[floor].iter_mut() {
        *it = Space::Full;
    }
    let mut count = 0;
    while put_sand(&mut field, SAND_SPAWN, 0, floor) {
        count += 1;
    }
    println!("[PART ONE]: {count}");

    while field[0][SAND_SPAWN] == Space::Empty {
        put_sand(&mut field, SAND_SPAWN, 0, floor);
        count += 1;
    }
    // Count the sand blocking the spawn point
    count += 1;

    println!("[PART TWO]: {count}");
}
