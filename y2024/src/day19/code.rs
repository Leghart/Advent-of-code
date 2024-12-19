use std::collections::HashMap;

#[allow(dead_code)]
fn load_data() -> (Vec<String>, Vec<String>) {
    let a = std::fs::read_to_string(file!().replace("/code.rs", "/data.txt")).unwrap();
    let b: Vec<&str> = a.split("\n\n").collect();
    let mut v1: Vec<String> = b[0].split(',').map(|s| s.trim().to_string()).collect();
    let v2: Vec<String> = b[1].lines().map(|s| s.trim().to_string()).collect();

    v1.sort_by(|a, b| b.len().cmp(&a.len()));
    (v1, v2)
}

pub fn part1() -> Option<usize> {
    let (towels, designs) = load_data();

    let mut cache: HashMap<String, bool> = HashMap::new();
    let mut count = 0;
    for desing in designs {
        if desing_is_ok_1(desing, &towels, &mut cache) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part2() -> Option<usize> {
    let (towels, designs) = load_data();

    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut count = 0;
    for desing in designs {
        count += desing_is_ok_2(desing, &towels, &mut cache);
    }

    Some(count)
}

fn desing_is_ok_1(design: String, towels: &Vec<String>, cache: &mut HashMap<String, bool>) -> bool {
    if cache.contains_key(&design) {
        return *cache.get(&design).unwrap();
    }
    if design == "" {
        return true;
    }
    let mut found = false;
    for towel in towels {
        if design.starts_with(towel) && {
            let sliced_design = &design[towel.len()..];
            desing_is_ok_1(sliced_design.to_string(), &towels, cache)
        } {
            found = true;
        }
    }
    cache.insert(design, found);
    found
}

fn desing_is_ok_2(
    design: String,
    towels: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if cache.contains_key(&design) {
        return *cache.get(&design).unwrap();
    }
    if design == "" {
        return 1;
    }
    let mut found = 0;
    for towel in towels {
        if design.starts_with(towel) {
            let sliced_design = &design[towel.len()..];
            found += desing_is_ok_2(sliced_design.to_string(), &towels, cache);
        }
    }
    cache.insert(design, found);
    found
}
