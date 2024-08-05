use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            return vec![]; // Retorna un laberinto vacío en caso de error
        }
    };

    let reader = BufReader::new(file);

    let maze: Vec<Vec<char>> = reader
        .lines()
        .map(|line| match line {
            Ok(line) => line.chars().collect(),
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                vec![] // En caso de error en la lectura de una línea, devuelve una fila vacía
            }
        })
        .collect();

    maze
}