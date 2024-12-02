#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn part1() -> Option<usize> {
    let data = load_data();
    let reports: Vec<Vec<i32>> = data
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut ctr = 0;

    for report in reports {
        let increasing: bool;
        if report[0] < report[1] {
            increasing = true;
        } else if report[0] > report[1] {
            increasing = false;
        } else {
            continue;
        }
        let mut was_break = false;
        for i in 0..report.len() - 1 {
            if increasing {
                let diff = report[i + 1] - report[i];
                if diff < 0 {
                    was_break = true;
                    break;
                }
                if !(1..4).contains(&diff) {
                    was_break = true;
                    break;
                }
            } else {
                let diff = -(report[i + 1] as i32 - report[i] as i32);
                if diff < 0 {
                    was_break = true;
                    break;
                }
                if !(1..4).contains(&diff) {
                    was_break = true;
                    break;
                }
            }
        }
        if !was_break {
            ctr += 1;
        }
    }

    Some(ctr)
}

pub fn part2() -> Option<usize> {
    let data = load_data();
    let reports: Vec<Vec<isize>> = data
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    let mut ctr = 0;

    for report in reports {
        for j in 0..report.len() {
            let mut vec_copy = report.clone();
            vec_copy.remove(j);

            if is_safe(&vec_copy) {
                ctr += 1;
                break;
            }
        }
    }
    Some(ctr)
}

fn is_safe(vec: &Vec<isize>) -> bool {
    let is_increasing = vec[1] > vec[0];

    if !(1..4).contains(&(vec[1] - vec[0]).abs()) {
        return false;
    }

    for i in 2..vec.len() {
        if (vec[i] > vec[i - 1]) != is_increasing {
            return false;
        }

        if !(1..4).contains(&(vec[i] - vec[i - 1]).abs()) {
            return false;
        }
    }

    true
}
