use std::path::Path;

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;

use crate::config;
use crate::maze::maze_phenotype::MazeCell;
use crate::maze::maze_phenotype::MazePhenotype;
use crate::maze::PathDirection;
use crate::simulator::SimulatorResult;
use crate::visualization::maze::{draw_maze, visualize_maze};

pub fn visualize_agent_path(maze: &MazePhenotype, simulator_result: &SimulatorResult, file_path: &Path) {
    let scale_u32 = 4 * config::MAZE.cell_dimension as u32;
    let mut drawing = RgbImage::new(maze.width * scale_u32 + 2, maze.height * scale_u32 + 2);

    draw_maze(maze, &mut drawing, scale_u32, false);
    draw_path(&mut drawing, maze, simulator_result);

    drawing.save(file_path).unwrap();
}

pub fn draw_path(drawing: &mut RgbImage, maze: &MazePhenotype, simulator_result: &SimulatorResult) {
    for (i, point) in simulator_result.agent_path.iter().enumerate() {
        println!("{} {}", point.x * config::MAZE.cell_dimension, point.y * config::MAZE.cell_dimension);


        draw_filled_circle_mut(
            drawing,
            (
                (point.x * 4.0 * config::MAZE.cell_dimension) as i32,
                ((maze.height as f64 - point.y) * 4.0 * config::MAZE.cell_dimension) as i32,
            ),
            2,
            Rgb([255, 0, 0]),
        );

        let mut zeros = "0000";

        if i < 10 {
            zeros = "000";
        } else if i < 100 {
            zeros = "00";
        } else if i < 1000 {
            zeros = "0";
        }

        drawing.save(format!("./agent/{}{}.png", zeros, i)).unwrap();
    }
}