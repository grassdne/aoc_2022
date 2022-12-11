use std::env;
use std::fs;
use std::process;
use std::str::Lines;

#[derive(Debug)]
enum Operation {
    Square,
    Add(u64),
    Mul(u64),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisible: u64,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
    inspect_count: u64,
}

const FMT_MONKEY:        &str = "Monkey ";
const FMT_STARTING_IEMS: &str = "  Starting items: ";
const FMT_OPERATION:     &str = "  Operation: new = ";
const FMT_TEST:          &str = "  Test: divisible by ";
const FMT_IF_TRUE:       &str = "    If true: throw to monkey ";
const FMT_IF_FALSE:      &str = "    If false: throw to monkey ";


fn build_operation(v: Vec<&str>) -> Option<Operation> {
    match v.as_slice() {
        ["old", "*", "old"] => Some(Operation::Square),
        ["old", "+", v]     => Some(Operation::Add(v.parse::<u64>().ok()?)),
        ["old", "*", v]     => Some(Operation::Mul(v.parse::<u64>().ok()?)),
        _ => None,
    }
}

fn build_monkey(lines: &mut Lines) -> Option<Monkey> {
    let items: Vec<u64> = lines.next()?
        .strip_prefix(FMT_STARTING_IEMS)?
        .split(", ")
        .map(|x| x.parse::<u64>().unwrap())
        .into_iter().collect();

    let operation = build_operation(lines.next()?.strip_prefix(FMT_OPERATION)?
        .split(" ")
        .collect::<Vec<&str>>())?;

    let test_divisible = lines.next()?
        .strip_prefix(FMT_TEST)?
        .parse::<u64>().ok()?;

    let if_true_throw_to = lines.next()?
        .strip_prefix(FMT_IF_TRUE)?
        .parse::<usize>().ok()?;

    let if_false_throw_to = lines.next()?
        .strip_prefix(FMT_IF_FALSE)?
        .parse::<usize>().ok()?;

    Some(Monkey { items, operation, test_divisible, if_true_throw_to, if_false_throw_to, inspect_count: 0 })
}

fn parse_monkey_file(file: &str) -> Vec<Monkey> {
    let mut lines = file.lines();
    let mut monkeys: Vec<Monkey> = Vec::new();
    loop {
        match lines.next()
        .map(|ln| ln.strip_prefix(FMT_MONKEY))
        {
            Some(Some(_)) => {
                monkeys.push(build_monkey(&mut lines).unwrap_or_else(|| {
                    eprintln!("Invalid input file: couldn't parse monkey");
                    process::exit(1);
                }));
            },
            Some(_) => {
                eprintln!("Invalid input file: couldn't parse line");
                process::exit(1);
            },
            None => break,
        }
        lines.next();
    }

    monkeys
}

fn perform_operation(old: u64, operation: &Operation) -> u64 {
    match operation {
        Operation::Square => old * old,
        Operation::Add(x) => old + x,
        Operation::Mul(x) => old * x,
    }
}

fn round(monkeys: &mut Vec<Monkey>, divby: Option<u64>, modby: Option<u64>) {
    for i in 0..monkeys.len() {
        let monke = &mut monkeys[i];
        let mut throws: Vec<(usize, u64)> = Vec::new();

        for item in &monke.items {
            monke.inspect_count += 1;
            let mut item = perform_operation(*item, &monke.operation);

            if let Some(x) = divby { item /= x; }
            if let Some(x) = modby { item %= x; }

            let throw_to = match item % monke.test_divisible {
                0 => monke.if_true_throw_to,
                _ => monke.if_false_throw_to
            };
            throws.push((throw_to, item));
        }

        for &(to, item) in throws.iter() {
            monkeys[to].items.push(item)
        }

        monkeys[i].items.clear();
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
    {
        let mut monkeys = parse_monkey_file(&content);
        for _i in 0..20 {
            round(&mut monkeys, Some(3), None);
        }
        let mut counts: Vec<u64> = monkeys.iter().map(|m| m.inspect_count).collect();
        counts.sort();
        println!("[PART ONE]: {}", counts.iter().rev().take(2).product::<u64>())
    }
    {
        let mut monkeys = parse_monkey_file(&content);
        let modby = monkeys.iter().map(|m| m.test_divisible).product::<u64>();
        for _i in 0..10_000 {
            round(&mut monkeys, None, Some(modby));
        }
        let mut counts: Vec<u64> = monkeys.iter().map(|m| m.inspect_count).collect();
        counts.sort();
        println!("[PART TWO]: {}", counts.iter().rev().take(2).product::<u64>());
    }
}
