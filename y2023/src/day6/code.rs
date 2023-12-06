#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn possible_solutions(time: isize, distance: isize) -> isize {
    let mut prev: isize = -1;
    let mut solutions: Vec<isize> = vec![];
    for (i, t) in (0..time).enumerate() {
        let result = t * (time - i as isize);
        if result == prev {
            return (solutions.len() * 2) as isize;
        }
        if result > distance {
            solutions.push(result);
        }
        prev = result;
    }
    solutions.len() as isize
}

pub fn part1() -> Option<usize> {
    let data: Vec<String> = load_data();
    let times: Vec<isize> = data
        .iter()
        .next()
        .unwrap()
        .split_whitespace()
        .skip_while(|&s| s != "Time:")
        .skip(1)
        .flat_map(|s| s.split_whitespace())
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<isize> = data
        .iter()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip_while(|&s| s != "Distance:")
        .skip(1)
        .flat_map(|s| s.split_whitespace())
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut results: Vec<usize> = vec![];

    let zipped = times.iter().zip(distances.iter());
    for (t, d) in zipped {
        let result = possible_solutions(t.clone(), d.clone()) as usize;
        results.push(result);
    }

    Some(results.iter().product())
}

pub fn part2() -> Option<usize> {
    let data: Vec<String> = load_data();
    let time = data
        .iter()
        .nth(0)
        .unwrap()
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect::<String>()
        .parse::<isize>()
        .unwrap();

    let distance = data
        .iter()
        .nth(1)
        .unwrap()
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect::<String>()
        .parse::<isize>()
        .unwrap();

    let mut results: Vec<usize> = vec![];

    let result = possible_solutions(time.clone(), distance.clone()) as usize;

    Some(result)
}
