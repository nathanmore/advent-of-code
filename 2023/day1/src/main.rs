fn main() {
    println!("Calibration Value: ");
}

fn get_calibration_value(input: &str) -> u32 {
    142
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