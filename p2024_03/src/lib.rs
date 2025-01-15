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

pub fn part1(input: &str) -> i64 {
    let mut res = 0;
    for line in input.trim().split('\n') {
        res += mult(line);
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut res = 0;
    let mut is_enabled = true;

    for line in input.trim().split('\n') {
        let mat: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        for s in mat {
            match s {
                "don't()" => { is_enabled = false; }
                "do()" => { is_enabled = true; }
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
    fn part1() {
        assert_eq!(super::part1(include_str!("sample.txt")), 161);
        assert_eq!(super::part1(include_str!("input.txt")), 171183089);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("sample.txt")), 48);
        assert_eq!(super::part2(include_str!("input.txt")), 63866497);
    }
}
