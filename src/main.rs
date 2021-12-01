use std::fs;

fn main() {
    println!("Hello, world!");

    println!("{}", day_one());
}

fn day_one() -> i32 {
    // read file
    let input = fs::read_to_string("inputs/one.txt").expect("Unable to read file");

    let input = input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut increases = 0;

    // if need to use enumerator so that we have the index
    for (i, _x) in input.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let prev = input[i - 1];
        let curr = input[i];
        if curr > prev {
            increases += 1;
        }
    }

    increases
}
