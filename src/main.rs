use std::fs;

fn main() {
    println!("Hello, world!");

    println!("Day 1: {}", day_one());
    println!("Day 2a: {}", day_three_a());
    // println!("Day 2b: {}", day_two_b());
}

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}
fn get_sig_bit(vec: &Vec<u8>) -> u8 {
    let mut ones = 0;
    let mut zeros = 0;

    for i in 0..vec.len() {
        if vec[i] == 1 {
            ones += 1;
        } else {
            zeros += 1;
        }
    }

    if ones > zeros { 1 } else { 0 }
}

fn day_three_a() -> i32 {
    let input = read_lines("inputs/three.txt");

    let first_five = &input[0..5];

    for num in first_five {
        println!("{}", num);
    }

    for i in 0..first_five.len() {
        let mut column = vec![];

        for j in 0..first_five.len() {
            column.push(first_five[j].chars().nth(i).unwrap());
        }
        println!("{}", get_sig_bit(&first_five[i].as_bytes().to_vec()));
    }

    // find gamma rate (most common bit of column)
    // let mut gamma_rate = 0;
    let binary_string = "".to_owned();

    // for each column in the first input line
    // for i in 0..input[0].len() {
    //     println!("{}: {}",i, input[i]);
    //     // collect all the values for that column
    //     let column = input.iter().map(|s| s.as_bytes()[i]).collect::<Vec<u8>>();

    //     // get the most common bit
    //     let bit = get_sig_bit(&column);

    //     // add it to the binary string
    //     binary_string.push_str(bit.to_string().chars().next().unwrap().to_string().as_str());
    // }

    println!("Binary String: {}", binary_string);


    // find epsilon rate 

    // find the power consumption (gamma * epsilon)

    1
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
