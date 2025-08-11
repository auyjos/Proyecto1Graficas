// maze.rs

use std::fs::File;
use std::io::{BufRead, BufReader};
use raylib::prelude::Vector2;

pub type Maze = Vec<Vec<char>>;

pub struct MazeData {
    pub maze: Maze,
    pub player_start: Vector2,
}

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn load_maze_with_player(filename: &str, block_size: usize) -> MazeData {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let maze: Maze = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    // Find player start position
    let mut player_start = Vector2::new(150.0, 150.0); // Default fallback
    
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                // Convert maze coordinates to world coordinates
                player_start = Vector2::new(
                    col_index as f32 * block_size as f32 + block_size as f32 / 2.0,
                    row_index as f32 * block_size as f32 + block_size as f32 / 2.0,
                );
                break;
            }
        }
    }

    MazeData {
        maze,
        player_start,
    }
}

