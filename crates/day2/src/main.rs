fn main() {
    let list: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let p1 = part1(&list);
    println!("p1: {:?}", p1);

    let p2 = part2(&list);
    println!("p2: {:?}", p2);
}

fn part1(reports: &Vec<Vec<u32>>) -> u32 {
    reports.into_iter().map(check_report).filter(|a| *a).count() as u32
}

fn part2(reports: &Vec<Vec<u32>>) -> u32 {
    reports
        .into_iter()
        .map(|r| {
            if check_report(r) {
                return true;
            }

            // handle a single failure by removing one element at a time
            // and check if the report is safe
            let mut i = 0;
            while i < r.len() {
                let mut r2 = r.clone();
                r2.remove(i);                
                if check_report(&r2) {
                    return true;
                }
                i += 1;
            }
            false
        })
        .filter(|a| *a)
        .count() as u32
}

// check if the report is safe
fn check_report(r: &Vec<u32>) -> bool {
    // check always increasing or all decreasing
    if r.is_sorted_by(|a, b| a < b) || r.is_sorted_by(|a, b| a > b) {
        // check rate change at most 3
        for i in 0..r.len() - 1 {
            if r[i].abs_diff(r[i + 1]) > 3 {
                return false;
            }
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let list: Vec<Vec<u32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let d = part1(&list);
        assert_eq!(d, 2);
    }

    #[test]
    fn test_second_part() {
        let list: Vec<Vec<u32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let d = part2(&list);
        assert_eq!(d, 4);
    }
}
