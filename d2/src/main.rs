use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let f = File::open("d2/src/input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut depth = 0;
    let mut horizontal = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<_> = line.split_ascii_whitespace().collect();
        match line[0] {
            "forward" => {
                horizontal += line[1].parse::<i32>().unwrap();
            }
            "up" => {
                depth -= line[1].parse::<i32>().unwrap();
            }
            "down" => {
                depth += line[1].parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }

    println!("part1: {}", depth * &horizontal);
}

fn part2() {
    let f = File::open("d2/src/input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<_> = line.split_ascii_whitespace().collect();
        match line[0] {
            "forward" => {
                let x = line[1].parse::<i32>().unwrap();
                horizontal += x;
                depth += aim * x;
            }
            "up" => {
                aim -= line[1].parse::<i32>().unwrap();
            }
            "down" => {
                aim += line[1].parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }

    println!("part2: {}", depth * &horizontal);
}
