use counter::Counter;
use std::collections::HashMap;

#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn cmp_by_cards(card1: &(String, u64, u64), card2: &(String, u64, u64)) -> std::cmp::Ordering {
    let weight = |c| match c {
        '2'..='9' => c as u32 - '0' as u32,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    };

    for (c1, c2) in card1.0.chars().zip(card2.0.chars()) {
        if weight(c1) == weight(c2) {
            continue;
        } else if weight(c1) < weight(c2) {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }
    !unreachable!();
}

fn cmp_by_cards2(card1: &(String, u64, u64), card2: &(String, u64, u64)) -> std::cmp::Ordering {
    let weight = |c| match c {
        '2'..='9' => c as u32 - '0' as u32,
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    };

    for (c1, c2) in card1.0.chars().zip(card2.0.chars()) {
        if weight(c1) == weight(c2) {
            continue;
        } else if weight(c1) < weight(c2) {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }
    !unreachable!();
}

pub fn part1() -> Option<u64> {
    let data = load_data();

    let mut ordered: Vec<(String, u64, u64)> = vec![];
    let strength_of_hands: HashMap<&str, u64> = HashMap::from_iter([
        ("high", 0),
        ("pair", 1),
        ("2pairs", 2),
        ("triple", 3),
        ("full", 4),
        ("four", 5),
        ("poker", 6),
    ]);

    for row in data.iter() {
        let mut splitted = row.split(" ");
        let cards = splitted.next().unwrap();
        let bid = splitted.next().unwrap().parse::<u64>().unwrap();
        let ctr = cards.chars().collect::<Counter<char>>();
        let hand = match ctr.values().len() {
            1 => "poker",
            5 => "high",
            4 => "pair",
            3 => {
                if *ctr.values().into_iter().max().unwrap() == 3 as usize {
                    "triple"
                } else {
                    "2pairs"
                }
            }
            2 => {
                if *ctr.values().into_iter().max().unwrap() == 4 as usize {
                    "four"
                } else {
                    "full"
                }
            }
            _ => !unreachable!(),
        };

        let hand_value = strength_of_hands.get(&hand).unwrap();
        ordered.push((cards.to_string(), bid, hand_value.to_owned()));
        ordered.sort_by(|a, b| {
            let compare_by_value = a.2.cmp(&b.2);

            if compare_by_value != std::cmp::Ordering::Equal {
                return compare_by_value;
            }

            cmp_by_cards(&a, &b)
        })
    }
    Some(
        ordered
            .iter()
            .enumerate()
            .map(|(index, &(_, second, _))| second * (index as u64 + 1))
            .sum::<u64>(),
    )
}

pub fn part2() -> Option<u64> {
    let data = load_data();

    let mut ordered: Vec<(String, u64, u64)> = vec![];
    let strength_of_hands: HashMap<&str, u64> = HashMap::from_iter([
        ("high", 0),
        ("pair", 1),
        ("2pairs", 2),
        ("triple", 3),
        ("full", 4),
        ("four", 5),
        ("poker", 6),
    ]);

    for row in data.iter() {
        let mut splitted = row.split(" ");
        let cards = splitted.next().unwrap();
        let bid = splitted.next().unwrap().parse::<u64>().unwrap();
        let mut ctr = cards.chars().collect::<Counter<char>>();

        let mut j_amount: usize = 0;

        if ctr.contains_key(&'J') {
            j_amount = *ctr.clone().get(&'J').unwrap();

            if 0 < j_amount && j_amount < 5 {
                ctr.remove(&'J');
                let most_common = ctr.most_common()[0];
                ctr[&most_common.0] += j_amount;
            }
        }

        let hand = match ctr.values().len() {
            1 => "poker",
            5 => "high",
            4 => "pair",
            3 => {
                if *ctr.values().into_iter().max().unwrap() == 3 as usize {
                    "triple"
                } else {
                    "2pairs"
                }
            }
            2 => {
                if *ctr.values().into_iter().max().unwrap() == 4 as usize {
                    "four"
                } else {
                    "full"
                }
            }
            _ => !unreachable!(),
        };

        let hand_value = strength_of_hands.get(&hand).unwrap();

        ordered.push((cards.to_string(), bid, hand_value.to_owned()));

        ordered.sort_by(|a, b| {
            let compare_by_value = a.2.cmp(&b.2);

            if compare_by_value != std::cmp::Ordering::Equal {
                return compare_by_value;
            }

            cmp_by_cards2(&a, &b)
        })
    }

    Some(
        ordered
            .iter()
            .enumerate()
            .map(|(index, &(_, second, _))| second * (index as u64 + 1))
            .sum::<u64>(),
    )
}
