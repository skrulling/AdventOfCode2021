use std::env;
use std::fs;

fn day_one(input: &[i32]) {
    let mut larger = 0;
    let mut last = -1;
    for number in input.iter() {
        if last != -1 && last < *number{
            larger += 1;
        }
        last = *number;
    }
    println!("Day one part one, {} are increased!", larger)
}

fn day_one_sums(input: &[i32]) {
    let mut larger = 0;
    let mut last = -1;
    let mut sum: i32;
    for (i, _number) in input.iter().enumerate() {
        sum = input[i] + input[i + 1] + input[i + 2];
        if last != -1 && last < sum {
            larger += 1;
        }
        last = sum;
        if i == input.len() - 3 {
            break;
        }
    }
    println!("Day one part two, {} are increased!", larger)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input_text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input = parse_to_int(input_text);


    day_one(&input);
    day_one_sums(&input);
}

fn parse_to_int(input_text: String) -> Vec<i32> {
    return input_text.lines()
        .map(|s| s.parse().unwrap()).collect();
}
