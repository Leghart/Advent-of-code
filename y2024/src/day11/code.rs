#[allow(dead_code)]
fn load_data() -> Vec<usize> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

pub fn part1() -> Option<usize> {
    let mut data = load_data();

    for _ in 0..25 {
        let mut after_blink: Vec<usize> = Vec::new();

        for el in data.clone() {
            if el == 0 {
                after_blink.push(1);
            } else if el.to_string().len() % 2 == 0 {
                let as_str = el.to_string();
                let len_str = as_str.len();
                let first = &as_str[..len_str / 2];
                let second = &as_str[len_str / 2..];

                after_blink.push(first.parse::<usize>().ok()?);
                after_blink.push(second.parse::<usize>().ok()?);
            } else {
                after_blink.push(el * 2024);
            }
        }
        data = after_blink.clone();
    }
    Some(data.len())
}

pub fn part2() -> Option<usize> {
    let mut data = load_data();

    for _ in 0..75 {
        let mut after_blink: Vec<usize> = Vec::new();

        for el in data.clone() {
            if el == 0 {
                after_blink.push(1);
            } else if el.to_string().len() % 2 == 0 {
                let as_str = el.to_string();
                let len_str = as_str.len();
                let first = &as_str[..len_str / 2];
                let second = &as_str[len_str / 2..];

                after_blink.push(first.parse::<usize>().ok()?);
                after_blink.push(second.parse::<usize>().ok()?);
            } else {
                after_blink.push(el * 2024);
            }
        }
        data = after_blink.clone();
    }
    Some(data.len())
}
