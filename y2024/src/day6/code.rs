use std::collections::HashSet;
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
    let mut pos: (isize, isize) = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                pos = (i as isize, j as isize);
            }
        }
    }

    let mut visited: HashSet<(isize, isize)> = HashSet::from([pos]);
    let mut direction: (isize, isize) = (-1, 0);

    let mut stop = false;
    while pos.0 >= 0 && pos.1 >= 0 {
        while {
            let new_row = pos.0 + direction.0;
            let new_col = pos.1 + direction.1;
            if (new_row as usize) >= matrix.len() || (new_col as usize) >= matrix[0].len() {
                stop = true;
            }
            new_row >= 0
                && new_col >= 0
                && !stop
                && matrix[new_row as usize][new_col as usize] != '#'
        } {
            if stop {
                break;
            }
            let new_row = pos.0 + direction.0;
            let new_col = pos.1 + direction.1;
            pos = (new_row, new_col);
            visited.insert(pos);
        }
        if stop {
            break;
        }
        direction = match direction {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => unreachable!(),
        };
    }
    Some(visited.len())
}

pub fn part2() -> Option<usize> {
    let matrix = load_data();
    let mut pos: (isize, isize) = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                pos = (i as isize, j as isize);
            }
        }
    }

    let mut count = 0;

    for i in 0..matrix.len() as isize {
        for j in 0..matrix[0 as usize].len() as isize {
            let mut npos = pos;
            let mut direction: (isize, isize) = (-1, 0);
            let mut visited: HashSet<((isize, isize), isize, isize)> = HashSet::new(); //direction, position
            loop {
                if visited.contains(&(direction, npos.0, npos.1)) {
                    count += 1;
                    break;
                }
                visited.insert((direction, npos.0, npos.1));
                let new_row = npos.0 + direction.0;
                let new_col = npos.1 + direction.1;

                if !((0 <= new_row && new_row < matrix.len() as isize)
                    && (0 <= new_col && new_col < matrix[0].len() as isize))
                {
                    break;
                }

                if matrix[new_row as usize][new_col as usize] == '#' || new_row == i && new_col == j
                {
                    direction = match direction {
                        (-1, 0) => (0, 1),
                        (0, 1) => (1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        _ => unreachable!(),
                    };
                } else {
                    npos.0 = new_row;
                    npos.1 = new_col;
                }
            }
        }
    }
    Some(count)
}
