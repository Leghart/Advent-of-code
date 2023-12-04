use std::collections::HashSet;

#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn part1() -> Option<usize> {
    let mut result = 0;
    for line in load_data() {
        let mut data = line.split(":").nth(1).unwrap().split("|");
        let winning: std::collections::HashSet<i32> = data
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let all: std::collections::HashSet<i32> = data
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let count = all.intersection(&winning).count() as u32;
        if count > 0 {
            result += 2u32.pow(count - 1);
        }
    }
    Some(result as usize)
}

struct Card {
    winning: HashSet<i32>,
    all: HashSet<i32>,
}

pub fn part2() -> Option<usize> {
    let mut cards: Vec<Card> = Vec::new();
    let data = load_data();

    for line in &data {
        let mut data = line.split(":").nth(1).unwrap().split("|");

        let winning: std::collections::HashSet<i32> = data
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let all: std::collections::HashSet<i32> = data
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        cards.push(Card { winning, all });
    }

    let mut multiplied = vec![1; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let matching = card.all.intersection(&card.winning).count();

        for _ in 0..multiplied[idx] {
            for i in idx + 1..idx + matching + 1 {
                if i != data.len() {
                    multiplied[i] += 1;
                }
            }
        }
    }

    Some(multiplied.iter().sum::<usize>())
}
