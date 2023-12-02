use std::fs;

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 12;
const BLUE_CUBES: u32 = 14;

fn main() {
    let contents = fs::read_to_string("day2/games.txt").unwrap_or("Invalid input".to_string());
    let sum = sum_possible_game_ids(&contents);
    print!("{sum}");
}

fn sum_possible_game_ids(contents: &str) -> u32 {
    let mut sum: u32 = 0;

    for lines in contents.lines() {
        let mut colon_iter = lines.split(":");
        let game_str = colon_iter.next().unwrap();
        let results = colon_iter.next().unwrap();
        
        // get id of game
        let mut game_split = game_str.split_whitespace();
        game_split.next();
        let id = game_split.next().unwrap().parse::<u32>().unwrap();

        
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        // get max of each color seen
        let draws_iter = results.split(";");
        for draw in draws_iter {
            let values = draw.split(",");
            for value in values {
                let mut iter = value.split(" ");
                iter.next();
                let num = iter.next().unwrap().parse::<u32>().unwrap();
                if value.contains("red") {
                    if num > red_max {
                        red_max = num;
                    }
                } else if value.contains("green") {
                    if num > green_max {
                        green_max = num;
                    }
                } else if value.contains("blue") {
                    if num > blue_max {
                        blue_max = num;
                    }
                }
            }
        }

        if red_max < RED_CUBES && green_max < GREEN_CUBES && blue_max < BLUE_CUBES {
            sum += id;
        }
    }
    sum
}

#[test]
fn check_possible_games() {
    let sum = sum_possible_game_ids(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    );

    assert_eq!(8, sum);
}