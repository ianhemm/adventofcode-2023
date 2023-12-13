use std::error::Error;

use crate::util::aoc_getdata;

pub fn aoc_daytwo() -> Result<String, Box<dyn Error>> {
    let input = aoc_getdata("02.txt")?;


    Ok(get_min_powers(&input).to_string())
}

fn _get_impossible(input: &String) -> u32 {
    let mut sum = 5050;
    for line in input.lines().into_iter() {
        let mut game_str = line.split(':');
        let game_id = game_str.next().unwrap().split_whitespace();
        let mut results = game_str.next().unwrap()
            .split(';')
            .filter(|game| { // get the specific handful
                let mut impossible = game.split(',').filter(|attempt|{ // get each cube in the handful
                    let mut result = attempt.split_whitespace();
                    let num_cubes: u32 = result.next().unwrap().parse().unwrap();
                    let color_cubes = result.next().unwrap();

                    match color_cubes { // if a specific handful has too many cubes, we want the attempt
                        "red" => num_cubes > 12,
                        "green" => num_cubes > 13,
                        "blue" => num_cubes > 14,
                        _ => false
                    }
                });
                if let Some(_) = impossible.next() { // if this list is not empty we dont want the attempt
                    true
                } else {
                    false
                }
            });

        if let Some(_) = results.next(){
            let mut game_iter = game_id;
            let _ = game_iter.next();
            let game_id: u32 = game_iter
                .next().unwrap()
                .parse().unwrap();

            sum -= game_id; // game isnt possible, so subtract its id FROM the sum
        }
    }

    sum
}

fn get_min_powers(input: &String) -> u32 {
    let mut sum = 0;
    for line in input.lines().into_iter() {
        let mut game_str = line.split(':');
        let mut _game_id = game_str.next().unwrap().split_whitespace();
        let game_results = game_str.next();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        game_results.unwrap()
            .split(';')
            .for_each(|game| { // get the specific handful
                let _ = game.split(',').for_each(|cube| {

                    let mut cube_split = cube.split_whitespace();
                    let cube_num = cube_split.next().unwrap();
                    let cube_color = cube_split.next().unwrap();

                    match cube_color{
                        "red" => red = max(&red, &cube_num.parse().unwrap()),
                        "green" => green = max(&green, &cube_num.parse().unwrap()),
                        "blue" => blue = max(&blue , &cube_num.parse().unwrap()),
                        _ => {}
                    };
                });
            });
        let power = red * green * blue;
        sum += power;
    }
    sum
}

fn max(a: &u32, b: &u32) -> u32 {
    if a > b {
        *a
    } else {
        *b
    }
}
