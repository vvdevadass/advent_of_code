use regex::Regex;

fn mult(line: &str) -> i64 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re_num = Regex::new(r"\d{1,3}").unwrap();
    let mut res = 0;
    let subs: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
    for s in subs {
        let mut mul = 1;
        for num in re_num.find_iter(s).map(|m| m.as_str().parse::<i64>().unwrap()) {
            mul *= num;
        }
        if mul > 1 {
            res+=mul;
        }
    }
    res
}

pub fn part_a(input: &str) -> i64 {
    let mut res = 0;
    for line in input.trim().split('\n') {
        res += mult(line);
    }
    res
}

pub fn part_b(input: &str) -> i64 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut res = 0;
    for line in input.trim().split('\n') {
        // let subs: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        // let subs = re.replace_all(line, "");
        // let s: &str = subs.borrow();
        // let subs2 = re2.replace_all(s, "");
        // println!("{}", subs);
        // res+=mult(subs2.into_owned().as_str());
        let mat: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        // println!("{:?}",mat);
        let mut is_enabled = true;
        for s in mat {
            match s {
                "don't()"=>{ is_enabled = false; }
                "do()"=>{ is_enabled = true; }
                _ => { 
                    if is_enabled {
                        res+=mult(s);
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 161);
        assert_eq!(super::part_a(include_str!("input.txt")), 171183089);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 48);
        // 53984975 < x < 69346806
        assert_eq!(super::part_b(include_str!("input.txt")), 53984975);
    }
}
