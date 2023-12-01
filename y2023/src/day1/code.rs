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
    let mut out: Vec<usize> = Vec::new();
    for line in data{
        let mut line_num:Vec<usize> = Vec::new();
        for c in line.chars(){
            if c.is_numeric(){
                line_num.push(c.to_digit(10).unwrap().try_into().unwrap());
            }            
        }
        let num = match line_num.len() {
            0 => {None}
            1 => Some(10*line_num[0]+line_num[0]),
            2 => Some(10*line_num[0]+line_num[1]),
            _ => Some(10*line_num[0]+line_num.last().unwrap()),
        };

        if num.is_some(){
            out.push(num.unwrap());
        }

    }
    Some(out.iter().sum())
}

pub fn part2() -> Option<usize> {
    let data = load_data();
    let mut out: Vec<usize> = Vec::new();
    for line in data{
        let mut line_num:Vec<usize> = Vec::new();
        let mut str_tmp: Vec<char> = Vec::new();
        for c in line.chars(){
            if c.is_numeric(){
                line_num.push(c.to_digit(10).unwrap().try_into().unwrap());
            } else {
                str_tmp.push(c);
                let i = match String::from_iter(&str_tmp).as_str(){
                    s if s.contains("one") => 1,
                    s if s.contains("two") => 2,
                    s if s.contains("three") => 3,
                    s if s.contains("four") => 4,
                    s if s.contains("five") => 5,
                    s if s.contains("six") => 6,
                    s if s.contains("seven") => 7,
                    s if s.contains("eight") => 8,
                    s if s.contains("nine") => 9,
                    _ => 0,
                };
                if i>0{
                    line_num.push(i);
                    str_tmp.clear();
                }
            }        
        }
        let num = match line_num.len() {
            0 => {None}
            1 => Some(10*line_num[0]+line_num[0]),
            2 => Some(10*line_num[0]+line_num[1]),
            _ => Some(10*line_num[0]+line_num.last().unwrap()),


        };

        if num.is_some(){
            out.push(num.unwrap());
        }

    }
    Some(out.iter().sum())
}
