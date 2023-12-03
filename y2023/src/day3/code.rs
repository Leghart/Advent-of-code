
#[allow(dead_code)]
fn load_data() -> Vec<String> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

// fn get_bounds()

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



fn asterix_near(f: usize, t: usize, data: &String, size: usize) -> bool {
    let symbol = &'*';
    let l_min: usize = 0;
    let r_max: usize = size - 1;

    //check left & right sides
    if f % size != l_min {
        if &data.chars().nth(f - 1).unwrap() == symbol {
            return true;
        }
    }
    if t % size != r_max {
        if &data.chars().nth(t).unwrap() == symbol {
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
            .any(|c| &c == symbol)
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
            .any(|c| &c == symbol)
        {
            return true;
        }
    }

    false
}


pub fn part2() -> Option<usize> {
    let pure_data = load_data();
    let size = pure_data.get(0)?.len();
    let data = pure_data.join("");

    let mut steps_to_skip = 0;
    let mut result:Vec<std::ops::Range<usize>> = vec![];
    // let a: Range<usize,usize> =

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


            if asterix_near(_from, _to, &data, size) {
                result.push(_from.._to);
            }
        }
    }   
    println!("{:?}",result) ;
    None
}
