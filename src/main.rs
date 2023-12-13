mod day_one;
mod day_two;
mod util;

#[allow(dead_code)]

fn main() {
    let answer = day_two::aoc_daytwo();
    println!("{}", answer.unwrap());
}

