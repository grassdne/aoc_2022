use std::cmp::Ordering;
use std::env;
use std::fs;
use std::process;
use std::str;
use std::vec;

#[derive(Debug, Clone)]
enum Item {
    List(Vec<Item>),
    Int(u32),
}

impl Item {
    fn parse_list(s: &str) -> Self {
        fn get_item(s: &str) -> (Item, &str) {
            if s.starts_with("[") {
                get_list(s)
            }
            else {
                let end = s.find(|x: char| !x.is_ascii_digit()).unwrap();
                (Item::Int(s[..end].parse::<u32>().unwrap()), &s[end..])
            }
        }
        fn get_list(mut s: &str) -> (Item, &str) {
            s = s.strip_prefix("[").unwrap();
            let mut items = Vec::new();
            while !s.starts_with("]") {
                let it;
                (it, s) = get_item(s);
                items.push(it);
                s = s.strip_prefix(",").unwrap_or(s);
            }
            s = s.strip_prefix("]").unwrap();
            (Item::List(items), s)
        }

        let (list, tail) = get_list(s.trim());
        assert_eq!(tail, "");
        list
    }

    fn cmp(&self, other: &Self) -> Ordering {
        use Item::*;
        match (self, other) {
            (Int(a), Int(b)) => a.cmp(b),
            (List(a), List(b)) => {
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.cmp(b))
                    .skip_while(|x| x.is_eq())
                    .next()
                    .unwrap_or(a.len().cmp(&b.len()))
            }

            (List(_), Int(b)) => {
                self.cmp(&List(vec![Int(*b)]))
            }

            (Int(_), List(_)) => {
                other.cmp(self).reverse()
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
    let _pairs: Vec<(Item, Item)> = content .split("\n\n")
                                            .map(|pair| pair .split_once("\n")
                                                             .map(|(a,b)| (Item::parse_list(a), Item::parse_list(b))).unwrap())
                                            .collect();

    println!("[PART ONE]: {}", _pairs.iter().enumerate()
                                .filter(|(_, (a,b))| a.cmp(b).is_lt())
                                .map(|(i, _)| i+1)
                                .sum::<usize>());


    let mut packets: Vec<Item> = content.lines().filter(|x| !x.is_empty())
                                            .map(|x| Item::parse_list(x))
                                            .collect();

    let divider_a = Item::List(vec![Item::List(vec![Item::Int(2)])]);
    let divider_b = Item::List(vec![Item::List(vec![Item::Int(6)])]);
    packets.push(divider_a.clone());
    packets.push(divider_b.clone());

    packets.sort_by(|a,b| a.cmp(b));
    println!("[PART TWO]: {}", (packets.iter().enumerate().find(|(_, p)| p.cmp(&divider_a).is_eq()).unwrap().0+1) *
                               (packets.iter().enumerate().find(|(_, p)| p.cmp(&divider_b).is_eq()).unwrap().0+1));
}
