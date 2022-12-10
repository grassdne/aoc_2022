use std::env;
use std::fs;

fn chdir(lines: &Vec<&str>, i: &mut usize, sizes: &mut Vec<u32>) -> u32 {
    let mut size = 0;
    while *i < lines.len() {
        let cmd = lines[*i];
        match &cmd[2..4] {
            "cd" => {
                *i += 1;
                if &cmd[5..] == ".." {
                    break;
                } else {
                    size += chdir(lines, i, sizes);
                }
            }
            "ls" => {
                while {*i += 1; *i < lines.len() } {
                    match lines[*i].split_once(' ').unwrap() {
                        ("dir", _name) => continue,
                        ("$", _cd) => break,
                        (sz, _name) => {
                            size += sz.parse::<u32>().unwrap()
                        },
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
    let lines: Vec<&str> = lines.lines().collect();
    let mut i: usize = 0;
    assert_eq!(lines[i], "$ cd /");
    i += 1;
    let mut sizes = Vec::new();
    let used_space = chdir(&lines, &mut i, &mut sizes);
    println!("[PART ONE]: {}", sizes.iter().filter(|&&x| x <= 100000).sum::<u32>());

    const TOTAL_DISK_SPACE: u32 = 70000000;
    const MIN_FREE_SPACE: u32 = 30000000;
    let free_space = TOTAL_DISK_SPACE - used_space;
    let needed_space = MIN_FREE_SPACE - free_space;
    
    println!("[PART TWO]: {}", sizes.iter().filter(|&&x| x >= needed_space).min().unwrap());
}
