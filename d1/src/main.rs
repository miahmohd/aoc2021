use std::{
    fs::File,
    io::{BufRead, BufReader},
};
// use std::io::Read;

fn main() {
    part1();
    part2();
}

fn part1() {
    let f = File::open("d1/src/input.txt").unwrap();
    let reader = BufReader::new(f);

    let res = reader
        .lines()
        .map(|r| r.unwrap())
        .map(|s| s.parse::<i32>().unwrap())
        .fold((0, 0), |accum, depth| {
            if depth > accum.1 {
                return (accum.0 + 1, depth);
            }
            return (accum.0, depth);
        });

    println!("part1: {}", res.0 - 1);
}

fn part2() {
    let f = File::open("d1/src/input.txt").unwrap();
    let reader = BufReader::new(f);

    let input: Vec<i32> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut res = 0;

    for i in 3..input.len() {
        if input[i] - input[i - 3] > 0 {
            res += 1;
        }
    }

    println!("part2: {}", res);
}
