#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn func(array: Vec<i32>) -> i32 {
    if array.iter().all(|x| *x == 0) {
        0
    } else {
        let diff_vector: Vec<i32> = array
            .clone()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        return *diff_vector.last().unwrap() + func(diff_vector.clone());
    }
}

pub fn part1() -> Option<i32> {
    let mut result = 0;

    for row in load_data() {
        let numbers: Vec<i32> = row
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        result += numbers.last().unwrap() + func(numbers.clone());
    }

    Some(result)
}

fn func2(array: Vec<i32>) -> i32 {
    if array.iter().all(|x| *x == 0) {
        0
    } else {
        let diff_vector: Vec<i32> = array
            .clone()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        let first_diff = func2(diff_vector.clone());
        return *diff_vector.first().unwrap() - first_diff;
    }
}

pub fn part2() -> Option<i32> {
    let mut result = 0;

    for row in load_data() {
        let numbers: Vec<i32> = row
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        result += numbers.first().unwrap() - func2(numbers.clone());
    }

    Some(result)
}
