use std::{process, fs, env};

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    x: i64,
    y: i64,
}

impl Vector2 {
    fn dist(self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    fn tuning_frequency(&self) -> i64 {
        self.x * 4000000 + self.y
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Vector2,
    beacon: Vector2,
}

impl Sensor {
    fn parse(ln: &str) -> Self {
        let ln = ln.strip_prefix("Sensor at x=").unwrap();
        let (x, ln) = ln.split_once(",")        .unwrap();
        let (y, ln) = ln.strip_prefix(" y=")    .unwrap()
                        .split_once(":")        .unwrap();

        let pos = Vector2 { x: x.parse().unwrap(), y: y.parse().unwrap() };

        let ln = ln.strip_prefix(" closest beacon is at x=").unwrap();
        let (x, ln) = ln.split_once(",").unwrap();
        let y = ln.strip_prefix(" y=").unwrap();
        let beacon = Vector2 { x: x.parse().unwrap(), y: y.parse().unwrap() };

        Self { pos, beacon }
    }
}

fn get_col_ranges_in_row(sensors: &Vec<Sensor>, row: i64) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for sensor in sensors {
        let rad = sensor.pos.dist(sensor.beacon);
        let ydist = sensor.pos.y.abs_diff(row) as i64;
        let xdist = rad - ydist;
        if xdist >= 0 {
            ranges.push((sensor.pos.x - xdist, sensor.pos.x + xdist));
        }
    }
    ranges.sort_by_key(|x| x.0);
    //println!("\nrow {row}");
    for i in 1..ranges.len() {
        ranges[i].0 = ranges[i].0.max(ranges[i-1].1);
        //println!("{:?}", &ranges[i]);
    }

    ranges
}

fn main() {
    let mut args = env::args().skip(1);
    let name = args.next().unwrap_or_else(|| {
        eprintln!("missing input file argument");
        process::exit(1);
    });
    let content = fs::read_to_string(&name).unwrap_or_else(|err| {
        eprintln!("failed reading file ({name}): {}", err);
        process::exit(1);
    });

    let (row, max_dist) = match name.find("sample") {
        Some(_) => (10, 20),
        None => (2000000, 4000000),
    };

    let sensors: Vec<Sensor> = content.lines().map(Sensor::parse).collect();

    {
        let ranges = get_col_ranges_in_row(&sensors, row);
        let mut total = 0;
        for &(s, e) in &ranges {
            if e > s {
                total += e - s;
            }
        }
        println!("[PART ONE]: {total}");
    }

    {
        'row_iter: for i in 0..=max_dist {
            let ranges = get_col_ranges_in_row(&sensors, i);

            let mut cur = 0;
            for &(s, e) in &ranges {
                if s > cur {
                    // We skipped an element!
                    println!("[PART TWO]: {}", Vector2{x: cur+1, y: i}.tuning_frequency());
                    break 'row_iter;
                }
                cur = cur.max(e);
                if cur > max_dist {
                    // We've checked the whole row!
                    break;
                }
            }
        }
    }
}
