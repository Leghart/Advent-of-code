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
    let mut count = 0;

    for line in data {
        let parts: Vec<&str> = line.split(": ").collect();
        let target = parts[0].parse::<usize>().unwrap();
        let nums: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        if is_correct1(nums[0].clone(), &nums[1..], target) {
            count += target;
        }
    }

    Some(count)
}

pub fn part2() -> Option<usize> {
    let data = load_data();
    let mut count = 0;

    for line in data {
        let parts: Vec<&str> = line.split(": ").collect();
        let target = parts[0].parse::<usize>().unwrap();
        let nums: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        if is_correct2(nums[0].clone(), &nums[1..], target) {
            count += target;
        }
    }

    Some(count)
}

fn is_correct2(calculated: usize, nums: &[usize], target: usize) -> bool {
    if calculated > target {
        return false;
    }

    if calculated == target && nums.len() == 0 {
        // assume: there is no 1 1 1 ... at the end
        return true;
    }

    if nums.len() == 0 {
        return false;
    }

    if is_correct2(calculated * nums[0], &nums[1..], target) {
        return true;
    }

    if is_correct2(calculated + nums[0], &nums[1..], target) {
        return true;
    }

    let merged = format!("{}{}", calculated, nums[0]);
    is_correct2(merged.parse::<usize>().unwrap(), &nums[1..], target)
}

fn is_correct1(calculated: usize, nums: &[usize], target: usize) -> bool {
    if calculated > target {
        return false;
    }

    if calculated == target && nums.len() == 0 {
        // assume: there is no 1 1 1 ... at the end
        return true;
    }

    if nums.len() == 0 {
        return false;
    }

    if is_correct1(calculated * nums[0], &nums[1..], target) {
        return true;
    }

    is_correct1(calculated + nums[0], &nums[1..], target)
}
