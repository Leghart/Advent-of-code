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
    let mut pairs: Vec<(usize, usize)> = data
        .iter()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .collect();

    let (mut col1, mut col2): (Vec<usize>, Vec<usize>) = pairs.into_iter().unzip();

    col1.sort();
    col2.sort();

    Some(
        col1.iter()
            .zip(&col2)
            .map(|(&a, &b)| (a as isize - b as isize).abs() as usize)
            .sum(),
    )
}

pub fn part2() -> Option<usize> {
    let data = load_data();
    let mut pairs: Vec<(usize, usize)> = data
        .iter()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .collect();

    let (mut col1, mut col2): (Vec<usize>, Vec<usize>) = pairs.into_iter().unzip();

    Some(
        col1.iter()
            .map(|&i| col2.iter().filter(|&x| *x == i).count() * i)
            .sum(),
    )
}
