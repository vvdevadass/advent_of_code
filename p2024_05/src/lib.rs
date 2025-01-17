pub fn part1(input: &str) -> i64 {
    let mut page_rules: Vec<Vec<&str>> = Vec::new();
    let mut res = 0;
    for line in input.trim().split('\n') {
        if line.contains("|") {
            let x = line.split("|").collect();
            page_rules.push(x);
        }
        else if line.contains(","){
            let pages: Vec<&str> = line.split(",").collect();
            for i in 0..pages.len()-1 {
                let s = vec![pages[i],pages[i+1]];
                if !page_rules.contains(&s) {
                    break;
                }

                if i == (pages.len()-2) {
                    let &x = pages.get(pages.len()/2).unwrap_or(&"0");
                    let x = x.parse::<i64>().unwrap();
                    println!("{}", x);
                    res+= x;
                }
            }
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let mut page_rules: Vec<Vec<&str>> = Vec::new();
    let mut res = 0;
    for line in input.trim().split('\n') {
        if line.contains("|") {
            let x = line.split("|").collect();
            page_rules.push(x);
        }
        else if line.contains(","){
            let mut pages: Vec<&str> = line.split(",").collect();
            let mut is_incorrect = false;
            for i in 0..pages.len()-1 {
                let s = vec![pages[i],pages[i+1]];
                let s2 = vec![pages[i+1],pages[i+1]];
                if !page_rules.contains(&s) && page_rules.contains(&s2) {
                    // break;
                    let a = pages[i+1];
                    pages[i+1] = pages[i];
                    pages[i] = a;
                    // res+=a.parse::<i64>().unwrap();
                    is_incorrect = true;
                }

                if ( i == (pages.len()-2) ) && is_incorrect {
                    let &x = pages.get(pages.len()/2).unwrap_or(&"0");
                    let x = x.parse::<i64>().unwrap();
                    println!("{}", x);
                    res+= x;
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
        assert_eq!(super::part1(include_str!("sample.txt")), 143);
        assert_eq!(super::part1(include_str!("input.txt")), 5108);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("sample.txt")), 123);
        assert_eq!(super::part2(include_str!("input.txt")), 0);
    }
}
