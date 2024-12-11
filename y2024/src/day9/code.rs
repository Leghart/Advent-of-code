#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect()
}

pub fn part1() -> Option<usize> {
    let data = load_data();

    let mut digits: Vec<String> = Vec::new();
    let mut id = 0;
    for (i, val) in data.into_iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..val.parse::<usize>().unwrap() {
                digits.push(id.to_string());
            }
            id += 1;
        } else {
            for _ in 0..val.parse::<usize>().unwrap() {
                digits.push('.'.to_string());
            }
        }
    }

    let mut rptr = digits.len() - 1;
    let mut lptr = 0;

    while lptr < rptr {
        if digits[lptr] == '.'.to_string() {
            while digits[rptr] == '.'.to_string() {
                rptr -= 1;
            }
            digits.swap(lptr, rptr);
            lptr += 1;
        } else {
            lptr += 1;
            continue;
        }
    }

    let mut count = 0;
    let mut idx = 0;
    loop {
        if digits[idx] == '.'.to_string() {
            break;
        }
        count += idx * digits[idx].parse::<usize>().unwrap();
        idx += 1;
    }

    Some(count)
}

fn are_next_blanks(digits: &Vec<String>, start: usize, window: usize) -> bool {
    for i in start..=(start + window) {
        if digits[i] != '.'.to_string() {
            return false;
        }
    }
    true
}

pub fn part2() -> Option<usize> {
    let data = load_data();

    let mut digits: Vec<String> = Vec::new();
    let mut id = 0;
    for (i, val) in data.into_iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..val.parse::<usize>().unwrap() {
                digits.push(id.to_string());
            }
            id += 1;
        } else {
            for _ in 0..val.parse::<usize>().unwrap() {
                digits.push('.'.to_string());
            }
        }
    }

    let mut rptr = digits.len() - 1;
    let mut stop = false;
    while !stop {
        let mut lptr = 0;

        if digits[rptr] != '.'.to_string() {
            let mut start_rptr = rptr;
            while digits[start_rptr] == digits[rptr] && rptr > 0 {
                rptr -= 1;
            }
            if rptr == 0 {
                stop = true;
                break;
            }
            rptr += 1; //correction
            let stop_rptr = rptr;
            let r_window = start_rptr - stop_rptr;

            let mut start_lptr: Option<usize> = None;
            loop {
                if digits[lptr] == '.'.to_string() && are_next_blanks(&digits, lptr, r_window) {
                    start_lptr = Some(lptr);
                    break;
                }
                if lptr > rptr {
                    break;
                }
                lptr += 1;
            }

            if let Some(left) = start_lptr {
                let mut _left = left;
                for _ in 0..=r_window {
                    digits.swap(_left, start_rptr);
                    start_rptr -= 1;
                    _left += 1;
                }
            }
        }
        rptr -= 1;
    }

    let mut count = 0;
    let mut idx = 0;

    while idx < digits.len() {
        if digits[idx] == '.'.to_string() {
            idx += 1;
            continue;
        }
        count += idx * digits[idx].parse::<usize>().unwrap();
        idx += 1;
    }

    Some(count)
}
