use std::path::Path;

use image::{Rgb, RgbImage};
use imageproc::drawing::{
    draw_cross_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut,
    draw_hollow_rect_mut, draw_line_segment_mut,
};
use imageproc::rect::Rect;

use crate::general::PathDirection;
use crate::maze_genotype::MazeGenome;
use crate::maze_phenotype::{MazeCell, MazePhenotype};

impl MazePhenotype {
    pub fn visualize(&self, file_path: &Path) {
        let mut scale = 20;
        let mut offset = 10;
        let radius = 2;

        let mut drawing = RgbImage::new(self.width * scale, self.height * scale);

        draw_filled_rect_mut(
            &mut drawing,
            Rect::at(0, 0).of_size(self.width * scale, self.height * scale),
            Rgb([255u8, 255u8, 255u8]),
        );

        let width: f32 = (self.width * scale - 1) as f32;
        let height: f32 = (self.height * scale - 1) as f32;

        self.draw_maze_borders(&mut drawing, width, height);

        for (x, row) in self.grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if cell.path_direction != PathDirection::None {
                    println!("{:#?}", cell);
                }
                self.draw_cell_borders(
                    &mut drawing,
                    cell,
                    (x * scale as usize) as f32,
                    (y * scale as usize) as f32,
                    scale as f32,
                );
                /*if cell.is_waypoint {
                    draw_filled_circle_mut(
                        &mut drawing,
                        (
                            (x * scale as usize + offset) as i32,
                            (y * scale as usize + offset) as i32,
                        ),
                        radius,
                        Rgb([0, 0, 0]),
                    );
                }
                if cell.is_juncture {
                    draw_filled_circle_mut(
                        &mut drawing,
                        (
                            (x * scale as usize + offset) as i32,
                            (y * scale as usize + offset) as i32,
                        ),
                        radius,
                        Rgb([0, 0, 0]),
                    );
                }
                if cell.path_direction != PathDirection::None {
                    draw_filled_circle_mut(
                        &mut drawing,
                        (
                            (x * scale as usize + offset) as i32,
                            (y * scale as usize + offset) as i32,
                        ),
                        radius,
                        Rgb([0, 0, 0]),
                    );
                }*/
            }
        }

        drawing.save(file_path).unwrap();
    }

    fn draw_maze_borders(&self, drawing: &mut RgbImage, width: f32, height: f32) {
        draw_line_segment_mut(drawing, (0.0, 0.0), (width, 0.0), Rgb([0, 0, 0]));
        draw_line_segment_mut(drawing, (width, 0.0), (width, height), Rgb([0, 0, 0]));
        draw_line_segment_mut(drawing, (0.0, height), (width, height), Rgb([0, 0, 0]));
        draw_line_segment_mut(drawing, (0.0, 0.0), (0.0, height), Rgb([0, 0, 0]));
    }

    fn draw_cell_borders(
        &self,
        drawing: &mut RgbImage,
        cell: &MazeCell,
        x: f32,
        y: f32,
        scale: f32,
    ) {
        if cell.north_wall {
            println!("drawing north: {}, {} to {}, {}", x, y, x + scale, y);
            draw_line_segment_mut(drawing, (x, y), (x + scale, y), Rgb([0, 0, 0]));
        }
        if cell.east_wall {
            println!(
                "drawing east: {}, {} to {}, {}",
                x + scale,
                y,
                x + scale,
                y + scale
            );
            draw_line_segment_mut(
                drawing,
                (x + scale, y),
                (x + scale, y + scale),
                Rgb([0, 0, 0]),
            );
        }
        if cell.south_wall {
            println!(
                "drawing south: {}, {} to {}, {}",
                x,
                y + scale,
                x + scale,
                y + scale
            );

            draw_line_segment_mut(
                drawing,
                (x, y + scale),
                (x + scale, y + scale),
                Rgb([0, 0, 0]),
            );
        }
        if cell.west_wall {
            println!("drawing west: {}, {} to {}, {}", x, y, x, y + scale);

            draw_line_segment_mut(drawing, (x, y), (x, y + scale), Rgb([0, 0, 0]));
        }
    }
}
