use std::{error::Error, collections::VecDeque};

use crate::util::aoc_getdata;

pub fn aoc_dayone() -> Result<String, Box<dyn Error>> {
    let match_str = [
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];
    let input = aoc_getdata("01.txt").unwrap();

    let mut out_num: u32 = 0;
    for line in input.lines() {

        let mut line = line.to_string();
        for entry in match_str.iter() {
            line = line.replace(&entry.0, &entry.1);
        }

        let mut nums = line
            .chars()
            .filter(|x| x.is_numeric())
            .map(|x| x.to_digit(10).unwrap())
            .collect::<VecDeque<u32>>();

        let first_num = nums.pop_front().unwrap();
        let second_num = nums.pop_back().unwrap_or_else(|| {
            first_num
        });
        let out_str = format!("{first_num}{second_num}");

        let num: u32 = out_str.parse()?;

        out_num += num;
    }

    let out = out_num.to_string();
    Ok(out)
}
