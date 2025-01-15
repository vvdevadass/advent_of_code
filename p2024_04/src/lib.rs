pub fn part1(input: &str) -> i64 {
    let mut arr: Vec<char> = Vec::new();
    let lines : Vec<&str> = input.lines().collect();
    let width: usize = lines.first().unwrap().len();
    let height: usize = lines.len();
    let mut res = 0;
    let mut s: String;

    let dirs = [
        "up",
        "down",
        "left",
        "right",
        "diagonal left up",
        "diagonal left down",
        "diagonal right up" ,
        "diagonal right down"
    ];

    for line in input.trim().split('\n') {
        for c in line.chars() {
            arr.push(c);
        }
    }

    for (i,&a) in arr.iter().enumerate() {
        if a == 'X' {
            let row: i32 = i as i32 /width as i32;
            let col: i32 = i as i32 %width as i32;
            let h = height as i32;
            let w = width as i32;
            let mut j: i32;
            let mut k: i32;
            let mut l: i32;

            for dir in dirs {
                // update j,k,l based on direction
                match  dir {
                    "up" => {
                        if row-3 >= 0 {
                            j = (row-1)*w + col;
                            k = (row-2)*w + col;
                            l = (row-3)*w + col;
                        }
                        else {
                            continue;
                        }
                    }
                    "down" => {
                        if row + 3 < h {
                            // down
                            j = (row+1)*w + col;
                            k = (row+2)*w + col;
                            l = (row+3)*w + col;
                        }
                        else {
                            continue;
                        }
                    }
                    "left" => {
                        if col - 3 >= 0 {
                            j = row*w + col-1;
                            k = row*w + col-2;
                            l = row*w + col-3;
                        }
                        else {
                            continue;
                        }
                    }
                    "right" => {
                        if col + 3 < w {
                            j = row*w + col+1;
                            k = row*w + col+2;
                            l = row*w + col+3;
                        }
                        else {
                            continue;
                        }
                    }
                    "diagonal left up" => {
                        if row - 3 >= 0 && col - 3 >= 0 {
                            j = (row - 1)*w + col-1;
                            k = (row - 2)*w + col-2;
                            l = (row - 3)*w + col-3;
                        }
                        else {
                            continue;
                        }
                    }
                    "diagonal left down" => {
                        if row + 3 < h && col - 3 >= 0 {
                            j = (row + 1)*w + col-1;
                            k = (row + 2)*w + col-2;
                            l = (row + 3)*w + col-3;
                        }
                        else {
                            continue;
                        }
                    }
                    "diagonal right up"  => {
                        if row - 3 >= 0 && col + 3 < w {
                            j = (row - 1)*w + col+1;
                            k = (row - 2)*w + col+2;
                            l = (row - 3)*w + col+3;
                        }
                        else {
                            continue;
                        }
                    }
                    "diagonal right down" => {
                        if row + 3 < h && col + 3 < w {
                            j = (row + 1)*w + col+1;
                            k = (row + 2)*w + col+2;
                            l = (row + 3)*w + col+3;
                        }
                        else {
                            continue;
                        }
                    }
                    _ => {
                        continue;
                    }
                }

                s = format!("{}{}{}{}", a, arr[j as usize], arr[k as usize], arr[l as usize]);
                if s.as_str() == "XMAS" {
                    res+=1;
                }
            }
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let lines : Vec<&str> = input.trim().lines().collect();
    let width: usize = lines.first().unwrap().len();
    let height: usize = lines.len();
    let target = "MAS";
    let target_rev = "SAM";
    let mut res = 0;

    for row in 0..(height-2) {
        for col in 0..(width-2) {
            let mut diag1 = String::new();
            let mut diag2 = String::new();

            for i in 0..3 {
                let char = lines[row+i].chars().nth(col+i).unwrap();
                diag1.push(char);

                let char = lines[row+i].chars().nth(col+2-i).unwrap();
                diag2.push(char);
            }

            if ( diag1.as_str() == target || diag1.as_str() == target_rev ) &&
               ( diag2.as_str() == target_rev || diag2.as_str() == target )
            {
                res+=1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("sample.txt")), 18);
        assert_eq!(super::part1(include_str!("input.txt")), 2573);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("sample.txt")), 9);
        assert_eq!(super::part2(include_str!("input.txt")), 1850);
    }
}
