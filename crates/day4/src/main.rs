fn main() {
    println!("Hello, world!");

    let word: Vec<char> = "XMAS".chars().collect::<Vec<char>>();

    let list= include_str!("input.txt").lines().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();

    let p1 = part1(&list, &word);
    println!("p1: {:?}", p1);

    let p2 = part2(&list);
    println!("p2: {:?}", p2);
}

// TODO: should treat this as a 1D array, would be far easier to search
fn part1(list: &Vec<Vec<char>>, word: &[char]) -> usize {
    let word_rev = word.iter().rev().copied().collect::<Vec<char>>();
    let mut count = 0usize;
    // search left and right
    for l in list {
        for i in 0..l.len() {
            // search foward
            if l[i..].starts_with(word) {
                println!(" left - right: {:?}", l);
                count += 1;
            }

            // search backward
            if l[i..].starts_with(&word_rev) {
                println!(" left - right rev: {:?}", l);
                count += 1;
            }
        }
    }

    // search up and down
    for i in 0..list[0].len() {
        let mut col = vec![];
        for l in list {
            col.push(l[i]);
        }
        for i in 0..col.len() {
            // search foward
            if col[i..].starts_with(word) {
                println!("updown: {:?}", col);
                count += 1;
            }

            // search backward
            if col[i..].starts_with(&word_rev) {
                println!("updown rev: {:?}", col);
                count += 1;
            }
        }
    }

    // search diagonally going down, left side
    for i in 0..list.len() {
        // Start from each row in the first column (left edge)
        let mut diag = vec![];
        let mut x = i;
        let mut y = 0;
        while x < list.len() && y < list[0].len() {
            diag.push(list[x][y]);
            x += 1;
            y += 1;
        }
        // Check the diagonal
        //println!("diag: {:?}", diag);
        for i in 0..diag.len() {
            // search forward
            if diag[i..].starts_with(word) {
                println!("diag down");
                count += 1;
            }

            // search backward
            if diag[i..].starts_with(&word_rev) {
                println!("diag down rev");
                count += 1;
            }
        }
    }

    // search diagonally going down, top side,
    for j in 1..list[0].len() {
        // Start from each column in the first row (top edge), excluding (0, 0)
        let mut diag = vec![];
        let mut x = 0;
        let mut y = j;
        while x < list.len() && y < list[0].len() {
            diag.push(list[x][y]);
            x += 1;
            y += 1;
        }
        // Check the diagonal
        //println!("diag: {:?}", diag);
        for i in 0..diag.len() {
            // search forward
            if diag[i..].starts_with(word) {
                println!("diag down");
                count += 1;
            }

            // search backward
            if diag[i..].starts_with(&word_rev) {
                println!("diag down rev");
                count += 1;
            }
        }
    }

    // search diagonally going up, left side
    for i in 0..list.len() {
        // Start from each row in the first column (left edge)
        let mut diag = vec![];
        let mut x = i;
        let mut y = 0; 
        while x < list.len() && y < list[0].len() {
            diag.push(list[x][y]);
            if x == 0 {
                break;
            }
            x -= 1;            
            y += 1;
        }
        // Check the diagonal
        //println!("diag: {:?}", diag);
        for i in 0..diag.len() {
            // search forward
            if diag[i..].starts_with(word) {
                println!("diag up");
                count += 1;
            }

            // search backward
            if diag[i..].starts_with(&word_rev) {
                println!("diag up rev");
                count += 1;
            }
        }
    }

    for j in 1..list[0].len() {
        // Start from each column in the last row (botton edge), excluding (0, n)
        let mut diag = vec![];
        let mut x = list.len() - 1;
        let mut y = j;
        while x < list.len() && y < list[0].len() {
            diag.push(list[x][y]);
            if x == 0 {
                break;
            }
            x -= 1;
            y += 1;
        }
        // Check the diagonal
        //println!("diag: {:?}", diag);
        for i in 0..diag.len() {
            // search forward
            if diag[i..].starts_with(word) {
                println!("diag up");
                count += 1;
            }

            // search backward
            if diag[i..].starts_with(&word_rev) {
                println!("diag up rev");
                count += 1;
            }
        }
    }


    count
}

fn part2(list: &Vec<Vec<char>>) -> usize {
    let mut count = 0usize;
    for x in 0..list.len()-2 {
        for y in 0..list[0].len() -2 {
            // A.B
            // .C.
            // D.E
            let a = list[x][y];
            let b = list[x][y+2];
            let c = list[x+1][y+1];
            let d = list[x+2][y];
            let e = list[x+2][y+2];
            if c == 'A' {
                if (a == 'M' && e == 'S') || (a == 'S' && e == 'M') {
                    if (b == 'M' && d == 'S') || (b == 'S' && d == 'M') {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_part() {
        let word: Vec<char> = "XMAS".chars().collect::<Vec<char>>();
        let data: Vec<Vec<char>> = vec!["..X...", ".SAMX.", ".A..A.", "XMAS.S", ".X...."]
            .iter()
            .map(|s| s.chars().collect())
            .collect();
        let d = part1(&data, &word);
        assert_eq!(d, 4);
    }

    #[test]
    fn first_part() {
        let word: Vec<char> = "XMAS".chars().collect::<Vec<char>>();
        let data: Vec<Vec<char>> = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
        .iter()
        .map(|s| s.chars().collect())
        .collect();
        assert_eq!(part1(&data, &word), 18);
        assert_eq!(part2(&data), 9);
    }

}
