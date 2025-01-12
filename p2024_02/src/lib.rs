pub fn part_a(input: &str) -> i64 {
    let mut res = 0;
    for line in input.trim().split('\n') {
        let strings: Vec<&str> = line.split(' ').collect();
        let v: Vec<i64> = strings.iter().flat_map(|x| x.parse()).collect();
        let mut dir: Option<bool> = None;
        let mut count = 0;
        for (i, &a) in v.iter().enumerate() {
            if i+1>=v.len() {
                break;
            }

            if (a-v[i+1]).abs() < 4 {
                if dir == None {
                    count+=1;
                    if a > v[i+1] {
                        dir = Some(false);
                    }
                    else if a < v[i+1] {
                        dir = Some(true);
                    }
                    else {
                        break;
                    }
                }
                else if dir == Some(false) {
                    if a > v[i+1] {
                        count+=1;
                        continue;
                    }
                    else {
                        break;
                    }
                }
                else {
                    if a < v[i+1] {
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

        if count == v.len()-1 {
            res+=1;
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
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 534);
    }

    
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 0);
    }
}
