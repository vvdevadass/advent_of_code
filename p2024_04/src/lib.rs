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
    let mut arr: Vec<char> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut res = 0;

    let dirs = [
        "vert M left",
        "vert M right",
        "horz M left" ,
        "horz M right"
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
        if a == 'M' {
            let row: i32 = i as i32 /width as i32;
            let col: i32 = i as i32 %width as i32;
            let h = height as i32;
            let w = width as i32;
            let mut j: i32; // second M
            let mut k: i32; // A
            let mut l: i32; // second S
            let mut n: i32; // first S
            // println!("{h},{w}");

            for dir in dirs {
                // update j,k,l based on direction
                match dir {
                    "diag left up"  => {
                        if row - 2 >= 0 && col - 2 >= 0 {
                            j = (row)*w + col-2;
                            k = (row - 1)*w + col-1;
                            l = (row - 2)*w + col-2;
                            n = (row - 2)*w + col;
                        }
                        else {
                            continue;
                        }
                    }
                    "diag left down"  => {
                        if row + 2 < h && col - 2 >= 0 {
                            j = (row + 2)*w + col;
                            k = (row + 1)*w + col-1;
                            l = (row + 2)*w + col-2;
                            n = (row)*w + col-2;
                        }
                        else {
                            continue;
                        }
                    }
                    "diag right up"  => {
                        if row - 2 >= 0 && col + 2 < w {
                            j = (row)*w + col+2;
                            k = (row - 1)*w + col+1;
                            l = (row - 2)*w + col+2;
                            n = (row - 2)*w + col;
                        }
                        else {
                            continue;
                        }
                    }
                    "diag right down" => {
                        if row + 2 < h && col + 2 < w {
                            // down
                            j = (row+2)*w + col;
                            k = (row+1)*w + col+1;
                            l = (row+2)*w + col+2;
                            n = (row)*w + col+2;
                        }
                        else {
                            continue;
                        }
                    }
                    _ => {
                        continue;
                    }
                }

                let s = format!("{}{}{}", a, arr[k as usize], arr[l as usize]);
                let s2 = format!("{}{}{}", arr[j as usize], arr[k as usize], arr[n as usize]);
                if s.as_str() == "MAS" && s2.as_str() == "MAS" {
                    res+=1;
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
        assert_eq!(super::part_a(include_str!("sample.txt")), 18);
        assert_eq!(super::part_a(include_str!("input.txt")), 2573);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 9);
        assert_eq!(super::part_b(include_str!("input.txt")), 0);
    }
}
