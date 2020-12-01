use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut numbers = Vec::new();

    for line in lines {
        if let Ok(num_str) = line {
            numbers.push(num_str.parse::<i32>().unwrap());
        }
    }

    // Part One

    for number in &numbers {
        let wanted = 2020 - number;
        if numbers.contains(&wanted) {
            println!("Part One: {}", wanted * number);
            break;
        }
    }

    // Part Two

    'outer: for x in &numbers {
        for y in &numbers {

            let wanted = 2020 - x - y;
            if numbers.contains(&wanted) {
                println!("Part Two: {}", wanted * x * y);
                break 'outer;
            }
        }
    }
    Ok(())
}
