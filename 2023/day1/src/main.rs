use std::fs;

fn main() {
    let contents = match fs::read_to_string("day1/input.txt") {
        Ok(s) => s,
        Err(_) => {
            println!("Invalid input");
            return ();
        }
    };
    let calibration = get_calibration_value(&contents);
    println!("The calibration value is {calibration}")
}

fn get_calibration_value(input: &str) -> u32 {
    let mut value: u32 = 0;
    for lines in input.lines() {
        let mut tens_value: Option<u32> = None;
        let mut ones_value: Option<u32> = None;

        for char in lines.chars() {
            match String::from(char).parse::<u32>() {
                Ok(n) => {
                    if tens_value == None {
                        tens_value = Some(n);
                    } else {
                        ones_value = Some(n);
                    }
                }
                _ => (),
            }
        }

        if ones_value == None {
            ones_value = tens_value;
        }

        value += (tens_value.unwrap() * 10) + ones_value.unwrap();
    }
    value
}

#[test]
fn example_input()
{
    let calibration = get_calibration_value(
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    );
    assert_eq!(142, calibration);
}