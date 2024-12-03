use regex::Regex;

#[allow(dead_code)]
fn load_data() -> String {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt").replace("\n", "")).unwrap()
}

pub fn part1() -> Option<usize> {
    let data = load_data();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let output = re
        .captures_iter(&data)
        .filter_map(|caps| {
            let x = caps.get(1)?.as_str().parse::<usize>().ok()?;
            let y = caps.get(2)?.as_str().parse::<usize>().ok()?;
            Some(x * y)
        })
        .sum();

    Some(output)
}

pub fn part2() -> Option<usize> {
    let data = load_data();

    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    let mut state = "do";
    let mut sum = 0;

    for caps in re.captures_iter(&data) {
        if let Some(matched) = caps.get(0) {
            let token = matched.as_str();
            if token == "do()" {
                state = "do";
            } else if token == "don't()" {
                state = "don't";
            } else if token.starts_with("mul(") && state == "do" {
                if let (Some(x_str), Some(y_str)) = (caps.get(2), caps.get(3)) {
                    let x = x_str.as_str().parse::<usize>().unwrap();
                    let y = y_str.as_str().parse::<usize>().unwrap();
                    let product = x * y;
                    sum += product;
                }
            }
        }
    }

    Some(sum)
}
