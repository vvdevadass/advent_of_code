pub fn part_a(input: &str) -> i64 {
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<i64> = Vec::new();
    let mut result = 0;
    for line in input.trim().split('\n') {
        //
        let strings: Vec<_> = line.split(' ').collect();
        let v : Vec<i64> = strings.iter().flat_map(|x| x.parse()).collect();
        v1.push(v[0]);
        v2.push(v[1]);
        // result += (vec[i][0] - vec[i][1]).abs();
    }
    v1.sort();
    v2.sort();
    for it in v1.iter().zip(v2.iter()) {
        let (a,b) = it;
        println!("{} {}",a,b);
        result += (a-b).abs();
    }

    result
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2378066);
    }
}
