use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn load_maze(filename: &str) -> Result<Vec<Vec<char>>> {
    let file = File::open(filename)
        .map_err(|e| {
            eprintln!("Error opening file {}: {}", filename, e);
            e
        })?;
    let reader = BufReader::new(file);

    let maze = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(maze)
}
