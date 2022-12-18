use std::{fmt, env, fs, process};
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
    Empty, Rock
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Jet {
    Left, Right
}

const CHAMBER_WIDTH: usize = 7;
const MIN_SPACE_BELOW: usize = 3;
const MIN_SPACE_LEFT: usize = 2;
const NUM_ROCK_TYPES: usize = 5;

impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..self.get_height()).rev() {
            for space in self.field[y] {
                f.write_char(match space {
                    Space::Empty => '.',
                    Space::Rock  => '#',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

struct Chamber {
    field: Vec<[Space; CHAMBER_WIDTH]>,
    rock_index: usize,
    rock_location: Option<(usize, usize)>,
    rock_types: [Vec<(usize, usize)>; 5],
    build_count: u64,
}

impl Chamber {
    fn new() -> Self {
        Self {
            field: Vec::new(),
            rock_index: 0,
            rock_types: Self::get_rock_types(),
            rock_location: None,
            build_count: 0,
        }
    }

    fn get_rock_types() -> [Vec<(usize, usize)>; NUM_ROCK_TYPES] {
        [
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        ]
    }

    fn rows(&self) -> std::slice::Iter<[Space; CHAMBER_WIDTH]> {
        self.field.iter()
    }

    fn ensure_rows(&mut self, rows: usize) {
        if self.field.len() < rows {
            self.field.resize(rows, [Space::Empty; CHAMBER_WIDTH])
        }
    }

    fn get_height(&self) -> usize {
        for (y, row) in self.rows().enumerate().rev() {
            if row.iter().find(|&&x| x == Space::Rock).is_some() {
                return y+1;
            }
        }
        return 0;
    }

    fn spawn_rock(&mut self) {
        let y = MIN_SPACE_BELOW + self.get_height();
        let x = MIN_SPACE_LEFT;
        self.ensure_rows(y + 4);
        self.rock_location = Some((x, y))
    }

    fn simulate_round(&mut self, jets: &mut impl Iterator<Item = Jet>) {
        if let Some((rock_x, rock_y)) = self.rock_location {
            let mut new_x = match jets.next() {
                Some(Jet::Left) => if rock_x > 0 { rock_x - 1 } else { 0 },
                Some(Jet::Right) => usize::min(rock_x+1, CHAMBER_WIDTH-1),
                None => panic!()
            };
            if self.rock_types[self.rock_index].iter()
                .find(|&&(x, y)| new_x + x >= CHAMBER_WIDTH || self.field[rock_y+y][new_x + x] == Space::Rock)
                .is_some()
            {
                new_x = rock_x;
            }
            if rock_y == 0
                || self.rock_types[self.rock_index].iter()
                    .find(|&&(x, y)| self.field[rock_y+y-1][new_x+x] == Space::Rock)
                    .is_some()
            {
                self.build_rocks(new_x, rock_y);
            }
            else {
                self.rock_location = Some((new_x, rock_y-1));
            }
        }
        else {
            self.spawn_rock();
        }
    }

    fn build_rocks(&mut self, pos_x: usize, pos_y: usize) {
        for &(x, y) in &self.rock_types[self.rock_index] {
            self.field[y+pos_y][x+pos_x] = Space::Rock;
        }
        self.rock_location = None;
        self.rock_index = (self.rock_index + 1) % NUM_ROCK_TYPES;
        self.build_count += 1;
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
    let mut jets = content.trim().chars().map(|b| {
        match b {
        '>' => Jet::Right,
        '<' => Jet::Left,
        _ => panic!("unexpected character in file")
        }
    }).cycle();

    let mut chamber = Chamber::new();
    while chamber.build_count < 2022 {
        chamber.simulate_round(&mut jets);
    }
    //chamber.build_rocks(chamber.rock_location.unwrap().0, chamber.rock_location.unwrap().1);
    println!("[PART ONE]: {}", chamber.get_height());
    //println!("{:?}", chamber);
}
