use glam::UVec2;
use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut map: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<_>>();

    let p1 = part1(&map);
    println!("p1: {:?}", p1);

    let p2 = part2(&mut map);
    println!("p2: {:?}", p2);
}

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub struct Location {
    pub position: UVec2,
    pub direction: Direction,
}

// possiable results of moving forward
pub enum ForwardMove {
    Blocker,
    Open(UVec2),
    Exit,
}

impl Location {
    // try to move forward, return new position if possible
    pub fn forward(&self, map: &Vec<Vec<char>>) -> ForwardMove {
        let mut x = self.position.x as i32;
        let mut y = self.position.y as i32;

        match self.direction {
            Direction::Up => x -= 1,
            Direction::Down => x += 1,
            Direction::Left => y -= 1,
            Direction::Right => y += 1,
        }
        // test we are still on the map
        if x >= map.len() as i32 || y >= map[0].len() as i32 || x < 0 || y < 0 {
            return ForwardMove::Exit;
        }

        // test we are on a open path
        if map[x as usize][y as usize] == '#' || map[x as usize][y as usize] == 'O' {
            return ForwardMove::Blocker;
        }

        let result = UVec2::new(x as u32, y as u32);
        //println!("{} -> {}", self.position, result);
        // return the new position
        ForwardMove::Open(result)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(map: &Vec<Vec<char>>) -> u32 {
    // store the locations we have been to with direction we where going at the time
    let mut locations: Vec<Location> = Vec::new();

    // find the starting position
    let mut guard = None;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '^' {
                guard = Some((x, y));
            }
        }
    }
    let (x, y) = guard.expect("Guard not found");
    let mut pos = Location {
        position: UVec2::new(x as u32, y as u32),
        direction: Direction::Up,
    };

    // loop until we repeat a location
    loop {
        //println!("{:?}", pos);
        if locations.contains(&pos) {
            // already been here
            break;
        }
        locations.push(pos.clone());
        // check if we can move forward
        match pos.forward(map) {
            ForwardMove::Blocker => {
                // turn right
                pos.direction = match pos.direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
            ForwardMove::Open(p) => pos.position = p,
            ForwardMove::Exit => break,
        }
    }

    // count unique locations
    let unique_locs = locations.iter().map(|x| x.position).collect::<HashSet<_>>();
    // print the map with the unique locations marked
    #[cfg(feature = "print")]
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let c = map[x][y];
            if unique_locs.contains(&UVec2::new(x as u32, y as u32)) {
                print!("X");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    unique_locs.len() as u32
}

fn part2(map: &mut Vec<Vec<char>>) -> usize {
    let mut count = 0;
    // find the starting position
    let mut guard = None;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let c = map[x][y];
            if c == '^' {
                guard = Some((x, y));
            }
        }
    }
    let (x, y) = guard.expect("Guard not found");
    let start_pos = Location {
        position: UVec2::new(x as u32, y as u32),
        direction: Direction::Up,
    };

    // search for loops
    #[cfg(not(feature = "parallel"))]
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '.' {
                // create new map
                map[x][y] = 'O';
                if does_map_loop(&start_pos, &map) {
                    count += 1;
                }
                map[x][y] = '.'; // reset
            }
        }
    }

    // search for loops in parallel
    #[cfg(feature = "parallel")]
    {
        // Collect all valid positions (x, y) to be processed
        let positions: Vec<(usize, usize)> = (0..map.len())
            .flat_map(|x| (0..map[0].len()).map(move |y| (x, y)))
            .filter(|&(x, y)| map[x][y] == '.')
            .collect();

        // Process positions in parallel
        count += positions
            .par_iter()
            .map(|&(x, y)| {
                let mut map_copy = map.clone(); // Clone the map for each thread
                map_copy[x][y] = 'O';
                let result = if does_map_loop(&start_pos, &map_copy) {
                    1
                } else {
                    0
                };
                result
            })
            .sum::<usize>();
    }
    count
}

fn does_map_loop(pos: &Location, map: &Vec<Vec<char>>) -> bool {
    let mut current = pos.clone();
    let mut locations: Vec<Location> = Vec::new();
    let mut found = false;
    // loop until we repeat a location
    loop {
        if locations.contains(&current) {
            // already been here
            found = true;
            break;
        }
        locations.push(current.clone());
        // check if we can move forward
        match current.forward(map) {
            ForwardMove::Blocker => {
                // turn right
                current.direction = match current.direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
            ForwardMove::Open(p) => current.position = p,
            ForwardMove::Exit => {
                break;
            }
        }
    }

    // print solutions
    #[cfg(feature = "print")]
    if found {
        let unique_locs = locations.iter().map(|x| x.position).collect::<HashSet<_>>();
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                if unique_locs.contains(&UVec2::new(x as u32, y as u32)) {
                    print!("X");
                } else {
                    print!("{}", map[x][y]);
                }
            }
            println!();
        }
        println!();
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let mut map: Vec<Vec<char>> = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ]
        .into_iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<_>>();

        //assert_eq!(part1(&map), 41);
        assert_eq!(part2(&mut map), 6);
    }
}
