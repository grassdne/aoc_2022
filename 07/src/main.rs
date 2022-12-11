use std::env;
use std::fs;
use std::str::Lines;
use std::iter::Peekable;

fn push_dir(lines: &mut Peekable<Lines>, sizes: &mut Vec<u32>) -> u32 {
    let mut size = 0;
    while lines.peek().is_some() {
        let cmd = lines.next().unwrap();
        match &cmd[2..4] {
            "cd" => {
                if cmd.ends_with("..") {
                    break;
                } else {
                    size += push_dir(lines, sizes);
                }
            }
            "ls" => {
                loop {
                    match lines.next_if(|ln| !ln.starts_with("$"))
                        .map(|ln| ln.split_once(' ').unwrap()) {
                        Some(("dir", _name)) => continue,
                        Some((sz, _name)) => size += sz.parse::<u32>().unwrap(),
                        None => break,
                    };
                }
            }
            _ => panic!("unknown command")
        };
    }
    sizes.push(size);
    size
}

fn main() {
    let name = env::args().nth(1)
        .expect("an input file argument");
    let lines = fs::read_to_string(name)
        .expect("able to open the input file");
    let mut lines = lines.lines().peekable();
    assert_eq!(lines.next(), Some("$ cd /"));
    let mut sizes = Vec::new();
    let used_space = push_dir(&mut lines, &mut sizes);
    println!("[PART ONE]: {}", sizes.iter().filter(|&&x| x <= 100000).sum::<u32>());

    const TOTAL_DISK_SPACE: u32 = 70000000;
    const MIN_FREE_SPACE: u32 = 30000000;
    let free_space = TOTAL_DISK_SPACE - used_space;
    let needed_space = MIN_FREE_SPACE - free_space;
    
    println!("[PART TWO]: {}", sizes.iter().filter(|&&x| x >= needed_space).min().unwrap());
}
