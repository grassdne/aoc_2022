use std::env;
use std::fs;
use std::process;


struct Tile {
    height: u8,
    shortest_dist: u32,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.height as char, self.shortest_dist))?;
        Ok(())
    }
}
type Field = Vec<Vec<Tile>>;

fn search(items: &mut Field, y: usize, x: usize, count: u32) {
    let cur = items[y][x].height;
    items[y][x].shortest_dist = count;

    let count = count + 1;

    for (i,j) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        // Please Rust I just want to subtract 1
        if let (Ok(i), Ok(j)) = (usize::try_from(y as i32 + i), usize::try_from(x as i32 + j)) {
            if let Some(Some(&Tile{height: e, shortest_dist: s})) = items.get(i).map(|r| r.get(j)) {
                if cur <= e + 1 && count < s {
                    search(items, i, j, count);
                }
            }
        }
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

    let mut start = None;
    let mut end = None;
    let mut items: Field = content.lines()
        .map(|ln| ln.as_bytes().iter().map(|&height| Tile{height, shortest_dist: u32::MAX})
            .collect::<Vec<Tile>>())
        .collect();

    for (y, row) in items.iter().enumerate() {
        for (x, &Tile{height: it, ..}) in row.iter().enumerate() {
            match it {
                b'S' => start = Some((y, x)),
                b'E' => end = Some((y, x)),
                _ => {}
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();
    items[start.0][start.1].height = b'a';
    items[end.0][end.1].height = b'z';
    search(&mut items, end.0, end.1, 0);
    println!("[PART ONE]: {}", &items[start.0][start.1].shortest_dist);
    println!("[PART TWO]: {}", items.iter()
                                    .flatten()
                                    .filter(|x| x.height == b'a')
                                    .map(|x| x.shortest_dist)
                                    .min()
                                    .unwrap())
}
