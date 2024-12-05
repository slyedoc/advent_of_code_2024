fn main() {    
    let list: Vec<(u32, u32)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let a: u32 = iter.next().unwrap().parse().unwrap();
            let b: u32 = iter.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect();

    let p1 = part1(list.clone());
    println!("p1: {:?}", p1);

    let p2 = part2(list.clone());
    println!("p2: {:?}", p2);
}

fn part1(list: Vec<(u32, u32)>) -> u32 {
    let (mut left, mut right) : (Vec<_>, Vec<_>) = list.into_iter()
    .map(|(a, b)| (a, b))
    .unzip();

    left.sort();
    right.sort();

    left.into_iter().zip(right.into_iter())
    .map(|(a, b)| a.abs_diff(b))
    .sum::<u32>()
}

fn part2(list: Vec<(u32, u32)>) -> u32 {
    let (left, right) : (Vec<_>, Vec<_>) = list.into_iter()
    .map(|(a, b)| (a, b))
    .unzip();

    left.into_iter().map(|a| a * right.iter().filter(|x| **x == a).count() as u32 )
    .sum::<u32>()    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let list: Vec<(u32, u32)> = vec![
            (3,4),
            (4,3),
            (2,5),
            (1,3),
            (3,9),
            (3,3),
        ];

        let d = part1(list);
        assert_eq!(d, 11);        
    }

    #[test]
    fn test_second_part() {
        let list: Vec<(u32, u32)> = vec![
            (3,4),
            (4,3),
            (2,5),
            (1,3),
            (3,9),
            (3,3),
        ];

        let d = part2(list);
        assert_eq!(d, 31);        
    }
}