use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let re = Regex::new(r"mul\([0-9]{1,3}\,[0-9]{1,3}\)").unwrap();
    let re_num = Regex::new(r"[0-9]{1,3},[0-9]{1,3}").unwrap();
    let mut res = 0;
    for line in input.trim().split('\n') {
        let subs: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        for s in subs {
            for num in re_num.find_iter(s).map(|m| m.as_str()) {
                let a : Vec<i64>= num.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
                let mut mul = 1;
                for i in a {
                    mul *=i;
                }
                if mul > 1 {
                    res+=mul;
                }
            }
        }
    }
    res
}

pub fn part_b(input: &str) -> i64 {
    for line in input.trim().split('\n') {
        //
    }
    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 161);
        assert_eq!(super::part_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 0);
    }
}
