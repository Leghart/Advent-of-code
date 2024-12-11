use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
fn load_data() -> Vec<Vec<char>> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1() -> Option<usize> {
    let matrix = load_data();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] != '.' {
                let sign = matrix[row][col];
                if !antennas.contains_key(&sign) {
                    antennas.insert(sign, Vec::new());
                }
                antennas.get_mut(&sign).map(|val| val.push((row, col)));
            }
        }
    }

    let mut count = 0;
    let mut unique: Vec<(usize, usize)> = Vec::new();

    for key in antennas.keys() {
        for comb in antennas[key].clone().into_iter().combinations(2) {
            let dx: isize = (comb[1].0 as isize - comb[0].0 as isize);
            let dy: isize = (comb[1].1 as isize - comb[0].1 as isize);

            let ddx = comb[0].0 as isize - dx;
            let ddy = comb[0].1 as isize - dy;

            if 0 <= ddx
                && ddx < matrix[0].len() as isize
                && 0 <= ddy
                && ddy < matrix.len() as isize
                && !unique.contains(&(ddx as usize, ddy as usize))
            {
                unique.push((ddx as usize, ddy as usize));
                count += 1;
            }

            let ddx = comb[1].0 as isize + dx;
            let ddy = comb[1].1 as isize + dy;
            if 0 <= ddx
                && ddx < matrix[0].len() as isize
                && 0 <= ddy
                && ddy < matrix.len() as isize
                && !unique.contains(&(ddx as usize, ddy as usize))
            {
                unique.push((ddx as usize, ddy as usize));
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part2() -> Option<usize> {
    let matrix = load_data();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] != '.' {
                let sign = matrix[row][col];
                if !antennas.contains_key(&sign) {
                    antennas.insert(sign, Vec::new());
                }
                antennas.get_mut(&sign).map(|val| val.push((row, col)));
            }
        }
    }

    let mut unique: Vec<(usize, usize)> = Vec::new();

    let mut empty_place_hits = 0;
    for key in antennas.keys() {
        for comb in antennas[key].clone().into_iter().combinations(2) {
            let dx: isize = (comb[1].0 as isize - comb[0].0 as isize);
            let dy: isize = (comb[1].1 as isize - comb[0].1 as isize);

            let mut tmp_dx: isize = comb[0].0 as isize;
            let mut tmp_dy: isize = comb[0].1 as isize;
            while is_in_range((tmp_dx as isize - dx, tmp_dy as isize - dy), &matrix) {
                let ddx = tmp_dx as isize - dx;
                let ddy = tmp_dy as isize - dy;

                if !unique.contains(&(ddx as usize, ddy as usize)) {
                    unique.push((ddx as usize, ddy as usize));
                    if matrix[ddx as usize][ddy as usize] == '.' {
                        empty_place_hits += 1;
                    }
                }
                tmp_dx = ddx as isize;
                tmp_dy = ddy as isize;
            }

            let mut tmp_dx = comb[1].0 as isize;
            let mut tmp_dy = comb[1].1 as isize;
            while is_in_range((tmp_dx as isize + dx, tmp_dy as isize + dy), &matrix) {
                let ddx = tmp_dx as isize + dx;
                let ddy = tmp_dy as isize + dy;
                if !unique.contains(&(ddx as usize, ddy as usize)) {
                    unique.push((ddx as usize, ddy as usize));
                    if matrix[ddx as usize][ddy as usize] == '.' {
                        empty_place_hits += 1;
                    }
                }
                tmp_dx = ddx as isize;
                tmp_dy = ddy as isize;
            }
        }
    }

    Some(antennas.iter().map(|(_, v)| v.len()).sum::<usize>() + empty_place_hits)
}

fn is_in_range(point: (isize, isize), matrix: &Vec<Vec<char>>) -> bool {
    if 0 <= point.0
        && point.0 < matrix[0].len() as isize
        && 0 <= point.1
        && point.1 < matrix.len() as isize
    {
        return true;
    }
    false
}
