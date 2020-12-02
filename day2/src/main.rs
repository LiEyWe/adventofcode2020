use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

struct DatabaseEntry {
    min: u8,
    max: u8,
    character: char,
    password: String,
}

impl DatabaseEntry {
    fn new(line: &str) -> Self {
        let tokens: Vec<&str> = line.splitn(2, ": ").collect();
        let password = tokens[1].to_string();
        let (min, max, character) = {
            let tokens: Vec<&str> = tokens[0].splitn(2, " ").collect();
            let character: char = tokens[1].chars().next().unwrap();
            let (min, max) = {
                let tokens: Vec<&str> = tokens[0].splitn(2, "-").collect();
                (
                    tokens[0].parse::<u8>().unwrap(),
                    tokens[1].parse::<u8>().unwrap(),
                )
            };

            (min, max, character)
        };

        Self {
            min,
            max,
            character,
            password,
        }
    }

    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&n| n == self.character)
            .count() as u8;
        self.min <= count && count <= self.max
    }

    fn is_valid_two(&self) -> bool {
        let pos1 = (self.min - 1) as usize;
        let pos2 = (self.max - 1) as usize;

        let mut count = 0;
        let password: Vec<char> = self.password.chars().collect();
        if password[pos1] == self.character {
            count += 1;
        }
        if password[pos2] == self.character {
            count += 1;
        }

        count == 1
    }
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut database_entries = Vec::new();
    for line in lines {
        if let Ok(entry_str) = line {
            database_entries.push(DatabaseEntry::new(&entry_str));
        }
    }

    // Part One

    let mut count = 0;
    for entry in &database_entries {
        if entry.is_valid() {
            count += 1;
        }
    }

    println!("Part One: {}", count);

    // Part Two

    count = 0;
    for entry in &database_entries {
        if entry.is_valid_two() {
            count += 1;
        }
    }

    println!("Part Two: {}", count);

    Ok(())
}
