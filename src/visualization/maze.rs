use std::path::Path;

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;

use crate::config;
use crate::maze::maze_phenotype::MazeCell;
use crate::maze::maze_phenotype::MazePhenotype;
use crate::maze::PathDirection;

#[allow(dead_code)]
pub fn visualize_maze(maze: &MazePhenotype, file_path: &Path, display_solution: bool) {
    let scale_u32 = 4 * config::MAZE.cell_dimension as u32;
    let mut drawing = RgbImage::new(maze.width * scale_u32 + 1, maze.height * scale_u32 + 1);

    draw_maze(maze, &mut drawing, scale_u32, display_solution);

    drawing.save(file_path).unwrap();
}

#[allow(dead_code)]
pub fn draw_maze(
    maze: &MazePhenotype,
    drawing: &mut RgbImage,
    scale_u32: u32,
    display_solution: bool,
) {
    let offset = config::MAZE.cell_dimension as usize / 2;
    let radius = 2;
    let scale_usize = 4 * config::MAZE.cell_dimension as usize;

    draw_filled_rect_mut(
        drawing,
        Rect::at(0, 0).of_size(maze.width * scale_u32 + 1, maze.height * scale_u32 + 1),
        Rgb([255u8, 255u8, 255u8]),
    );

    for (x, column) in maze.grid.iter().enumerate() {
        for (y, cell) in column.iter().rev().enumerate() {
            draw_cell_borders(
                drawing,
                cell,
                (x * scale_usize) as f32,
                (y * scale_usize) as f32,
                scale_usize as f32,
            );

            if display_solution {
                if cell.is_waypoint {
                    draw_filled_circle_mut(
                        drawing,
                        (
                            (x * scale_usize + offset) as i32,
                            (y * scale_usize + offset) as i32,
                        ),
                        radius,
                        Rgb([0, 0, 0]),
                    );
                }
                if cell.is_juncture {
                    draw_filled_circle_mut(
                        drawing,
                        (
                            (x * scale_usize + offset) as i32,
                            (y * scale_usize + offset) as i32,
                        ),
                        radius,
                        Rgb([0, 0, 0]),
                    );
                }
                if cell.path_direction != PathDirection::None {
                    draw_filled_circle_mut(
                        drawing,
                        (
                            (x * scale_usize + offset) as i32,
                            (y * scale_usize + offset) as i32,
                        ),
                        radius,
                        Rgb([0, 0, 0]),
                    );
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn draw_cell_borders(drawing: &mut RgbImage, cell: &MazeCell, x: f32, y: f32, scale: f32) {
    if cell.north_wall {
        draw_line_segment_mut(drawing, (x, y), (x + scale, y), Rgb([0, 0, 0]));
    }
    if cell.east_wall {
        draw_line_segment_mut(
            drawing,
            (x + scale, y),
            (x + scale, y + scale),
            Rgb([0, 0, 0]),
        );
    }
    if cell.south_wall {
        draw_line_segment_mut(
            drawing,
            (x, y + scale),
            (x + scale, y + scale),
            Rgb([0, 0, 0]),
        );
    }
    if cell.west_wall {
        draw_line_segment_mut(drawing, (x, y), (x, y + scale), Rgb([0, 0, 0]));
    }
}
