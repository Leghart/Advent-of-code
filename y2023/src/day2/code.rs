#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
#[derive(Debug, PartialEq, Eq, Hash)]

enum Cube {
    red,
    blue,
    green,
}
#[derive(Debug)]

struct Draw {
    pub color: Cube,
    pub amount: usize,
}

impl PartialEq for Draw {
    fn eq(&self, other: &Self) -> bool {
        (&self.color, self.amount) == (&other.color, other.amount)
    }
}
impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.amount.cmp(&other.amount))
    }
}
#[derive(Debug)]
struct Game {
    pub id: usize,
    pub sets: Vec<Vec<Draw>>,
}

fn line_to_structs(line: String) -> Game {
    let mut parts = line.split(':');
    let mut sets: Vec<Vec<Draw>> = vec![];
    let game_number: usize;

    if let (Some(game_number_str), Some(game_data)) = (parts.next(), parts.next()) {
        game_number = game_number_str
            .trim()
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap();
        for set in game_data.split(';').into_iter() {
            let splitted: Vec<&str> = set.split(",").collect();
            let mut tmp_vec: Vec<Draw> = vec![];

            for draw in splitted {
                let pure_draw: Vec<&str> = draw.trim().split(" ").collect();
                let amount = pure_draw[0].parse::<usize>().unwrap();
                let color = match pure_draw[1] {
                    "red" => Cube::red,
                    "blue" => Cube::blue,
                    "green" => Cube::green,
                    _ => unreachable!(),
                };
                tmp_vec.push(Draw { amount, color });
            }
            sets.push(tmp_vec);
        }
    } else {
        !unreachable!()
    };

    Game {
        id: game_number,
        sets: sets,
    }
}

pub fn part1() -> Option<usize> {
    let data = load_data();
    let mut sum_ids = 0;

    let rules: std::collections::HashMap<Cube, usize> = std::collections::HashMap::from_iter([
        (Cube::red, 12),
        (Cube::green, 13),
        (Cube::blue, 14),
    ]);

    for line in data {
        let game = line_to_structs(line);
        let mut instant_break = false;
        for set in game.sets {
            if instant_break {
                break;
            }

            for draw in set {
                if draw.amount > *rules.get(&draw.color).unwrap() {
                    instant_break = true;
                    break;
                }
            }
        }
        if !instant_break {
            sum_ids += game.id;
        }
    }
    Some(sum_ids)
}

pub fn part2() -> Option<usize> {
    let data = load_data();
    let mut multiplied: Vec<usize> = vec![];

    for line in data {
        let game = line_to_structs(line);
        let mut min_values: std::collections::HashMap<Cube, usize> =
            std::collections::HashMap::from_iter([
                (Cube::red, 0),
                (Cube::green, 0),
                (Cube::blue, 0),
            ]);

        for set in game.sets {
            for draw in set {
                if min_values.get(&draw.color).unwrap() < &draw.amount {
                    *min_values.get_mut(&draw.color).unwrap() = draw.amount;
                }
            }
        }
        multiplied.push(min_values.values().product::<usize>());
    }
    Some(multiplied.iter().sum())
}
