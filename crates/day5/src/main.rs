fn main() {
    let input = include_str!("input.txt");

    let mut rules: Vec<(u32, u32)> = vec![];
    let mut manuals: Vec<Vec<u32>> = vec![];

    let mut on_manuals = false;
    for l in input.lines() {
        if l.is_empty() {
            on_manuals = true;
            continue;
        }
        if on_manuals {
            println!("{:?}", l);
            let manual: Vec<u32> = l.split(',').map(|x| x.parse().unwrap()).collect();
            manuals.push(manual);
        } else {
            let rule: Vec<u32> = l.split('|').map(|x| x.parse().unwrap()).collect();
            rules.push((rule[0], rule[1]));
        }
    }

    let p1 = part1(&rules, &manuals);
    println!("p1: {:?}", p1);



}

fn part1(rules: &Vec<(u32, u32)>, manuals: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    'outer: for m in manuals {
        // check out of order pages
        for (i, current) in m.iter().enumerate() {            
            // check if any rules would put any remaining pages before the current page
            for j in i+1..m.len() {
                let next = m[j];
                // check if any would would reverse this order
                if rules.iter().any(|(a, b)| {
                    if *a == next && *b == *current {
                        return true;
                    }
                    false                    
                }) {
                    // we are out of order
                    continue 'outer;            
                }
            }            
        }
        // we are correct
        //println!("{:?}", m);
        sum += m[m.len() / 2];
    }
    sum
}

fn part2(rules: &Vec<(u32, u32)>, manuals: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    'outer: for m in manuals {
        // check out of order pages
        let mut out_of_order = false;
        for (i, current) in m.iter().enumerate() {            

            // check if any rules would put any remaining pages before the current page
            for j in i+1..m.len() {
                let next = m[j];
                // check if any would would reverse this order
                if rules.iter().any(|(a, b)| {
                    if *a == next && *b == *current {
                        return true;
                    }
                    false                    
                }) {
                    // we are out of order
                    out_of_order = true;
                }
            }            
        }

        if !out_of_order {
            break;
        }

        // TODO: fix order

        // we are correct
        println!("{:?}", m);        
        sum += m[m.len() / 2];
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let rules: Vec<(u32, u32)> = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];

        let manuals: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(part1(&rules, &manuals), 143);
        assert_eq!(part2(&rules, &manuals), 123);
        //assert_eq!(part2(&data), 9);
    }
}
