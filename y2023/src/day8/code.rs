use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn part1() -> Option<usize> {
    return None;
    let mut data = load_data();

    let steps = data.remove(0);
    data.remove(0);
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for row in data {
        let splitted: Vec<&str> = row.split("=").collect();
        let key = splitted[0].trim();
        let re = Regex::new(r"\(([^,]+), ([^)]+)\)").unwrap();

        if let Some(captures) = re.captures(row.as_str()) {
            if let (Some(l), Some(r)) = (captures.get(1), captures.get(2)) {
                map.insert(
                    key.to_string(),
                    (l.as_str().to_string(), r.as_str().to_string()),
                );
            }
        }
    }

    let mut result = 0;
    let mut start = "AAA".to_string();

    for step in steps.chars().cycle() {
        if start == "ZZZ".to_string() {
            break;
        }

        match step {
            'R' => {
                let msg = format!("error with value {:?}", start);
                start = map.get(&start).expect(msg.as_str()).1.clone();
            }
            'L' => {
                start = map.get(&start).unwrap().0.clone();
            }
            _ => unreachable!(),
        };
        result += 1;
    }

    Some(result)
}

pub fn part2() -> Option<usize> {
    let mut data = load_data();

    let steps = data.remove(0);
    data.remove(0);
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for row in data {
        let splitted: Vec<&str> = row.split("=").collect();
        let key = splitted[0].trim();
        let re = Regex::new(r"\(([^,]+), ([^)]+)\)").unwrap();

        if let Some(captures) = re.captures(row.as_str()) {
            if let (Some(l), Some(r)) = (captures.get(1), captures.get(2)) {
                map.insert(
                    key.to_string(),
                    (l.as_str().to_string(), r.as_str().to_string()),
                );
            }
        }
    }

    let mut start_nodes: Vec<&String> = map
        .keys()
        .filter(|x| x.chars().nth(2).unwrap() == 'A')
        .clone()
        .collect();

    let mut results: Vec<usize> = vec![];

    for start_node in start_nodes {
        let mut start = start_node.clone();
        let mut result = 0;
        for step in steps.chars().cycle() {
            if start == "Z".to_string() {
                break;
            }

            match step {
                'R' => {
                    start = map.get(&start).unwrap().1.clone();
                }
                'L' => {
                    start = map.get(&start).unwrap().0.clone();
                }
                _ => unreachable!(),
            };
            result += 1;
        }
        results.push(result);
    }
    // Some(result)
    println!("{:?}", results);
    // println!("LCM {:?}", lcm())
    None
}
