#![feature(substr_range)]
use core::panic;

use regex::Regex;

fn main() {
    let list= include_str!("input.txt");

    let p1 = part1(&list);
    println!("p1: {:?}", p1);

    let p2 = part2(&list);
    println!("p2: {:?}", p2);
}

fn part1(lines: &str) -> usize {
    let mut results: Vec<(usize, usize)> = vec![];
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // find matches for the pattern
    for (_, [x_str,y_str]) in re.captures_iter(lines).map(|c| c.extract()) {
        let (x,y): (usize, usize) = (x_str.parse().unwrap(),y_str.parse().unwrap());
        if x > 999 || y > 999 {
            panic!("Values too large");
        }
        results.push((x,y));
    }
    results.into_iter().map(|(x,y)| x*y).sum()    
}

fn part2(data: &str) -> usize {
    let mut results: Vec<(usize, usize)> = vec![];
    let mut delete_ranges: Vec<(usize, usize)> = vec![];
    // find any dont sections
    let dont = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    dont.captures_iter(data).for_each(|c| {    
        let range = c.get(0).unwrap().range();
        //println!("dont: {:?} {:?}", range, data[range.start..range.end].to_string());
        delete_ranges.push((range.start, range.end));
    });

    // remove the dont sections, start at end to avoid offset issues
    let mut lines = data.to_string();    
    for (start, end) in delete_ranges.iter().rev() {            
       //println!( "{:?} {:?} {:?}", start, end, lines[*start..*end].to_string());
       lines.replace_range(start..end, "");        
    }

    // check for any dont at the end of the string
    let dont = Regex::new(r"don't\(\).*$").unwrap();
    if let Some(c) = dont.captures(&lines) {
        println!("remote end: {:?}", c.get(0).unwrap().range());
        let range = c.get(0).unwrap().range();
        lines.replace_range(range.start..range.end, "");
    }
    println!("{:?}", lines);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // find matches for the pattern
    for (s, [x,y]) in re.captures_iter(&lines).map(|c| c.extract()) {
        println!("{:?} {:?} {:?}", x, y, s);
        results.push((x.parse().unwrap(),y.parse().unwrap()));
    }
    //println!("{:?}", results);
    results.into_iter().map(|(x,y)| x*y).sum()    
}

#[allow(dead_code)]
fn part2_stolen(data: &str) -> usize {
    let mut res = 0usize;
    let mut valid = true;
    let re = Regex::new(r"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(?<dt>don't\(\))|(?<d>do\(\))")
    .unwrap();
    for m in re.captures_iter(data) {
        if let Some(_) = m.name("d") {
            valid = true;
        } else if let Some(_) = m.name("dt") {
            valid = false;
        }
        if valid {
            if let (Some(op1), Some(op2)) = (m.name("op1"), m.name("op2")) {
                res += op1.as_str().parse::<usize>().unwrap() * op2.as_str().parse::<usize>().unwrap();
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let list = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let d = part1(&list);
        assert_eq!(d, 161);
    }

    #[test]
    fn second_part() {
        let list = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        println!("{:?}", list);
        let d = part2(&list);
        assert_eq!(d, 48);
    }
}