fn safety_check(report: &Vec<i64> ) -> bool {
    let mut safe = false;
    let mut dir: Option<bool> = None;
    let mut count = 0;
    for (i, &a) in report.iter().enumerate() {
        if i+1>=report.len() {
            break;
        }

        if (a-report[i+1]).abs() < 4 {
            if dir == None {
                count+=1;
                if a > report[i+1] {
                    dir = Some(false);
                }
                else if a < report[i+1] {
                    dir = Some(true);
                }
                else {
                    break;
                }
            }
            else if dir == Some(false) {
                if a > report[i+1] {
                    count+=1;
                    continue;
                }
                else {
                    break;
                }
            }
            else {
                if a < report[i+1] {
                    count+=1;
                    continue;
                }
                else {
                    break;
                }
            }
        }
        else {
            break;
        }
    }
    
    if count == report.len()-1 {
        safe=true;
    }
    safe
}

pub fn part1(input: &str) -> i64 {
    let mut res = 0;
    for line in input.trim().split('\n') {
        let strings: Vec<&str> = line.split(' ').collect();
        let v: Vec<i64> = strings.iter().flat_map(|x| x.parse()).collect();
        if safety_check(&v) == true {
            res+=1;
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let mut res = 0;
    for line in input.trim().split('\n') {
        let strings: Vec<&str> = line.split(' ').collect();
        let v: Vec<i64> = strings.iter().flat_map(|x| x.parse()).collect();
        if safety_check(&v) == true {
            res+=1;
        }
        else {
            for (i,_) in v.iter().enumerate() {
                let mut w = v.clone();
                w.remove(i);
                if safety_check(&w) == true {
                    res+=1;
                    break;
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
        assert_eq!(super::part2(include_str!("sample.txt")), 2);
        assert_eq!(super::part1(include_str!("input.txt")), 534);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("sample.txt")), 4);
        assert_eq!(super::part2(include_str!("input.txt")), 577);
    }
}
