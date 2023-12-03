#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn is_symbol_near(f: usize, t: usize, data: &String, size: usize) -> bool {
    let symbols = ['#', '*', '+', '$', '/', '@', '=', '%', '&', '-'];
    let l_min: usize = 0;
    let r_max: usize = size - 1;

    //check left & right sides
    if f % size != l_min {
        if symbols.contains(&data.chars().nth(f - 1).unwrap()) {
            return true;
        }
    }
    if t % size != r_max {
        if symbols.contains(&data.chars().nth(t).unwrap()) {
            return true;
        }
    }

    //check top side
    if !(l_min..=r_max).contains(&f) {
        let ff = if (f - size - 1) % size == r_max {
            f - size
        } else {
            f - size - 1
        };

        let tt = if (t - size + 1) % size == l_min {
            t - size
        } else {
            t - size + 1
        };

        if data
            .get(ff..tt)
            .unwrap()
            .chars()
            .any(|c| symbols.contains(&c))
        {
            return true;
        }
    }

    //check bottom side
    if !(data.len() - size..=data.len()).contains(&f) {
        let ff = if (f + size - 1) % size == r_max {
            f + size
        } else {
            f + size - 1
        };

        let tt = if (t + size + 1) % size == l_min {
            t + size
        } else {
            t + size + 1
        };

        if data
            .get(ff..tt)
            .unwrap()
            .chars()
            .any(|c| symbols.contains(&c))
        {
            return true;
        }
    }

    false
}

pub fn part1() -> Option<usize> {
    return None;
    let pure_data = load_data();
    let size = pure_data.get(0)?.len();
    let data = pure_data.join("");

    let mut steps_to_skip = 0;
    let mut result: usize = 0;

    for (i, c) in data.chars().enumerate() {
        let _from: usize = i;
        let mut _to: usize = i;

        if steps_to_skip > 0 {
            steps_to_skip -= 1;
            continue;
        }

        if c.is_digit(10) {
            while data.chars().collect::<Vec<char>>()[_to].is_digit(10) {
                _to += 1;
            }
            steps_to_skip = _to - _from;
            if is_symbol_near(_from, _to, &data, size) {
                result += data.get(_from.._to).unwrap().parse::<usize>().unwrap();
            }
        }
    }
    Some(result)
}

fn deep_number(idx: usize, data: &String) -> usize {
    let mut idx_start = idx;
    while idx_start > 0 && data.chars().nth(idx_start - 1).unwrap().is_digit(10) {
        idx_start -= 1;
    }

    let mut idx_stop = idx_start;
    while data.chars().nth(idx_stop + 1).unwrap().is_digit(10) {
        idx_stop += 1;
    }

    data.get(idx_start..idx_stop + 1)
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn find_near_numbers(idx: i32, data: &String, size: i32) -> Vec<usize> {
    let mut numbers: Vec<usize> = vec![];
    let directions: [i32; 8] = [
        -1 * (size + 1),
        -1 * size,
        -1 * (size - 1),
        -1,
        1,
        size - 1,
        size,
        size + 1,
    ];
    for n in directions {
        let nidx = n + idx;
        if nidx >= 0
            && nidx <= data.len() as i32
            && data.chars().nth(nidx as usize).unwrap().is_digit(10)
        {
            let number = deep_number(nidx as usize, &data);
            if !numbers.contains(&number) {
                numbers.push(number);
            }
        }
    }
    numbers
}

pub fn part2() -> Option<usize> {
    let pure_data = load_data();
    let size = pure_data.get(0)?.len();
    let data = pure_data.join("");

    let mut result = 0;

    for (i, c) in data.chars().enumerate() {
        if c == '*' {
            let numbers = find_near_numbers(i as i32, &data, size as i32);
            if numbers.len() == 2 {
                result += numbers.iter().product::<usize>();
            }
        }
    }

    Some(result)
}
