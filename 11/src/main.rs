use std::env;
use std::fs;
use std::process;
use std::str::Lines;

#[derive(Debug)]
enum OpValue {
    Num(u64),
    Old,
}

#[derive(Debug)]
enum Op {
    Times, Plus
}

#[derive(Debug)]
struct Operation {
    left: OpValue,
    op: Op,
    right: OpValue,
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

fn build_opvalue(left: &str) -> Option<OpValue> {
    match left {
        "old" => Some(OpValue::Old),
        n => Some(OpValue::Num(n.parse::<u64>().ok()?)),
    }
}

fn build_op(op: &str) -> Option<Op> {
    match op {
        "*" => Some(Op::Times),
        "+" => Some(Op::Plus),
        _   => None,
    }
}

fn build_operation(v: Vec<&str>) -> Option<Operation> {
    match v.as_slice() {
        [left, op, right] => {
            Some(Operation {
                left: build_opvalue(left)?,
                right: build_opvalue(right)?,
                op: build_op(op)?
            })
        }
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

fn opvalue_substitute(old: u64, opval: &OpValue) -> u64 {
    match opval {
        OpValue::Old => old,
        OpValue::Num(n) => *n,
    }
}

fn perform_operation(old: u64, operation: &Operation) -> u64 {
    let left = opvalue_substitute(old, &operation.left);
    let right = opvalue_substitute(old, &operation.right);
    match operation.op {
        Op::Plus => left + right,
        Op::Times => left * right,
    }
}

fn round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        //println!("Monkey {i}");
        let monke = &mut monkeys[i];
        let mut throws: Vec<(usize, u64)> = Vec::new();
        for item in &monke.items {
            //println!("  Monkey inspects an item with worry level {item}");
            monke.inspect_count += 1;
            let item = perform_operation(*item, &monke.operation) / 3;
            //println!("  Worry level becomes {item}");
            let throw_to = match item % monke.test_divisible {
                0 => monke.if_true_throw_to,
                _ => monke.if_false_throw_to
            };
            //println!("  Item with worry level {item} thrown to {throw_to}");
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
    if true {
        let mut monkeys = parse_monkey_file(&content);
        for _i in 0..20 {
            round(&mut monkeys);
        }
        let mut counts: Vec<u64> = monkeys.iter().map(|m| m.inspect_count).collect();
        counts.sort();
        println!("[PART ONE]: {}", counts.iter().rev().take(2).product::<u64>())
    }
}
