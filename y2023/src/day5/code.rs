#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn part1() -> Option<i64> {
    let mut pure_data = load_data();
    let seeds: Vec<i64> = pure_data
        .remove(0)
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let data: Vec<&String> = pure_data.iter().filter(|c| !c.is_empty()).collect();

    let mut maps: Vec<Vec<Vec<i64>>> = vec![];

    for row in data {
        if row.contains("map") {
            maps.push(Vec::new());
        } else if !row.is_empty() {
            let values: Vec<i64> = row.split_whitespace().map(|s| s.parse().unwrap()).collect();
            maps.last_mut().unwrap().push(values);
        }
    }

    let mut current: i64;
    let mut result: Vec<i64> = vec![];
    for seed in seeds {
        current = seed;
        for map in &maps {
            for range in map {
                if (range[1]..(range[1] + range[2])).contains(&current) {
                    current = current - (range[1] - range[0]);
                    break;
                }
            }
        }
        result.push(current);
    }
    Some(result.iter().min().unwrap().to_owned())
}

pub fn part2() -> Option<i64> {
    None
}
