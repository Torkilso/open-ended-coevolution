use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;

use crate::config;
use crate::maze::maze_phenotype::MazePhenotype;
use crate::neatns::novelty_archive::NoveltyArchive;
use crate::simulator::SimulatorResult;
use crate::visualization::maze::draw_maze;
use crate::visualization::VisualizationOptions;
use std::path::Path;

pub fn visualize_agent_path(
    maze: &MazePhenotype,
    simulator_result: &SimulatorResult,
    options: VisualizationOptions,
) {
    let scale_u32 = 4 * config::MAZE.cell_dimension as u32;
    let mut drawing = RgbImage::new(maze.width * scale_u32 + 2, maze.height * scale_u32 + 2);

    draw_maze(maze, &mut drawing, scale_u32, false);
    draw_path(&mut drawing, maze, simulator_result, &options);

    let file_path = format!("{}/{}", options.folder_path, options.file_name);

    let path = Path::new(&file_path);
    drawing.save(path).unwrap();
}

pub fn draw_path(
    drawing: &mut RgbImage,
    maze: &MazePhenotype,
    simulator_result: &SimulatorResult,
    options: &VisualizationOptions,
) {
    for (_, point) in simulator_result.agent_path.iter().enumerate() {
        draw_filled_circle_mut(
            drawing,
            (
                (point.x * 4.0 * config::MAZE.cell_dimension) as i32,
                ((maze.height as f64 - point.y) * 4.0 * config::MAZE.cell_dimension) as i32,
            ),
            2,
            Rgb([255, 0, 0]),
        );

        /*let mut zeros = "000000";

        if i < 10 {
            zeros = "00000";
        } else if i < 100 {
            zeros = "0000";
        } else if i < 1000 {
            zeros = "000";
        } else if i < 10000 {
            zeros = "00";
        } else if i < 100000 {
            zeros = "0";
        }

        drawing
            .save(format!("./novelty/{}{}.png", zeros, i))
            .unwrap();

        if i == 99999 {
            break;
        }*/
    }
}
