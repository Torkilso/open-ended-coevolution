use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;

use crate::config;
use crate::maze::maze_phenotype::MazePhenotype;
use crate::neatns::novelty_archive::NoveltyArchive;
use crate::simulator::SimulatorResult;
use crate::visualization::maze::draw_maze;

#[allow(dead_code)]
pub fn visualize_agent_path(maze: &MazePhenotype, simulator_result: &SimulatorResult) {
    let scale_u32 = 4 * config::MAZE.cell_dimension as u32;
    let mut drawing = RgbImage::new(maze.width * scale_u32 + 2, maze.height * scale_u32 + 2);

    draw_maze(maze, &mut drawing, scale_u32, false);
    draw_path(&mut drawing, maze, simulator_result);
}

#[allow(dead_code)]
pub fn draw_path(drawing: &mut RgbImage, maze: &MazePhenotype, simulator_result: &SimulatorResult) {
    for (i, point) in simulator_result.agent_path.iter().enumerate() {
        //println!("{} {}", point.x * config::MAZE.cell_dimension, point.y * config::MAZE.cell_dimension);

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

#[allow(dead_code)]
pub fn draw_novelty_archive(maze: &MazePhenotype, novelty_archive: &NoveltyArchive) {
    let scale_u32 = 4 * config::MAZE.cell_dimension as u32;
    let mut drawing = RgbImage::new(maze.width * scale_u32 + 2, maze.height * scale_u32 + 2);

    draw_maze(maze, &mut drawing, scale_u32, false);
    draw_archive(&mut drawing, maze, novelty_archive);

    //drawing.save(file_path).unwrap();
}

#[allow(dead_code)]
pub fn draw_archive(
    drawing: &mut RgbImage,
    maze: &MazePhenotype,
    novelty_archive: &NoveltyArchive,
) {
    for (i, point) in novelty_archive.novelty_items.iter().enumerate() {
        //println!("{} {}", point.x * config::MAZE.cell_dimension, point.y * config::MAZE.cell_dimension);

        draw_filled_circle_mut(
            drawing,
            (
                (point.x * 4.0 * config::MAZE.cell_dimension) as i32,
                ((maze.height as f64 - point.y) * 4.0 * config::MAZE.cell_dimension) as i32,
            ),
            2,
            Rgb([255, 0, 0]),
        );

        let mut zeros = "00000";

        if i < 10 {
            zeros = "0000";
        } else if i < 100 {
            zeros = "000";
        } else if i < 1000 {
            zeros = "00";
        } else if i < 10000 {
            zeros = "0";
        }

        drawing
            .save(format!("./novelty/{}{}.png", zeros, i))
            .unwrap();

        if i == 9999 {
            break;
        }
    }
}
