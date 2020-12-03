use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct SlopeMap {
    contents: Vec<Vec<char>>,
}

impl SlopeMap {
    fn new(contents: Vec<Vec<char>>) -> Self {
        Self { contents }
    }

    fn advance_right(&mut self, right: usize) {
        for map_line in &mut self.contents {
            map_line.rotate_left(right);
        }
    }

    fn advance_down(&mut self, down: usize) {
        let mut down = down;
        if down > self.contents.len() {
            down = self.contents.len();
        }
        self.contents.drain(..down);
    }

    fn advance(&mut self, right: usize, down: usize) {
        self.advance_right(right);
        self.advance_down(down);
    }

    fn is_tree(&self) -> bool {
        if self.has_finished() {
            return false;
        }
        *self.contents.first().unwrap().first().unwrap() == '#'
    }

    fn has_finished(&self) -> bool {
        self.contents.is_empty()
    }
}

fn count_with_slope(slope_map: &mut SlopeMap, right: usize, down: usize) -> usize {
    let mut count = 0;
    while !slope_map.has_finished() {
        slope_map.advance(right, down);
        if slope_map.is_tree() {
            count += 1;
        }
    }

    count
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut slope_map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        if let Ok(map_str) = line {
            slope_map.push(map_str.chars().collect());
        }
    }

    let slope_map = SlopeMap::new(slope_map);

    // Part One

    let num_trees = count_with_slope(&mut slope_map.clone(), 3, 1);

    println!("Part One: {}", num_trees);

    // Part Two

    let result = count_with_slope(&mut slope_map.clone(), 1, 1)
        * count_with_slope(&mut slope_map.clone(), 3, 1)
        * count_with_slope(&mut slope_map.clone(), 5, 1)
        * count_with_slope(&mut slope_map.clone(), 7, 1)
        * count_with_slope(&mut slope_map.clone(), 1, 2);
    println!("Part Two: {}", result);

    Ok(())
}
