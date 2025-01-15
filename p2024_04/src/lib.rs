pub fn part_a(input: &str) -> i64 {
    let mut arr: Vec<char> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
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
        if width == 0 {
            width = line.len();
        }
        for c in line.chars() {
            arr.push(c);
        }
        height+=1;
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
        assert_eq!(super::part_a(include_str!("sample.txt")), 18);
        assert_eq!(super::part_a(include_str!("input.txt")), 2573);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 0);
        assert_eq!(super::part_b(include_str!("input.txt")), 0);
    }
}
