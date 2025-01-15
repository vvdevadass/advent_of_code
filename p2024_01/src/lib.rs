
pub fn part1(input: &str) -> i64 {
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<i64> = Vec::new();
    let mut result = 0;
    for line in input.trim().split('\n') {
        let strings: Vec<_> = line.split(' ').collect();
        let v : Vec<i64> = strings.iter().flat_map(|x| x.parse()).collect();
        v1.push(v[0]);
        v2.push(v[1]);
    }
    v1.sort();
    v2.sort();
    for it in v1.iter().zip(v2.iter()) {
        let (a,b) = it;
        result += (a-b).abs();
    }

    result
}

pub fn part2(input: &str) -> i64 {
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<i64> = Vec::new();
    let mut result = 0;
    for line in input.trim().split('\n') {
        let strings: Vec<_> = line.split(' ').collect();
        let v : Vec<i64> = strings.iter().flat_map(|x| x.parse()).collect();
        v1.push(v[0]);
        v2.push(v[1]);
    }
    v1.sort();
    v2.sort();
    for a in v1 {
        let r = v2.iter().filter(|&&x| x == a).count();
        result += a*(r as i64);
    }

    result
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part1("\n"), 0);
    // }

    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("input.txt")), 2378066);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("input.txt")), 18934359);
    }
}
