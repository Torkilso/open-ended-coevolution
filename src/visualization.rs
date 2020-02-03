use crate::maze::{MazePhenotype, MazeGenome, MazeCell};
use draw::*;
use draw::shape::LinePoint;

fn add_borders(c: &mut Canvas, height: u32, width: u32) {
    let mut top = Drawing::new()
        .with_shape(Shape::Rectangle { height: 1, width: width * 20 })
        .with_xy(0.0, 0.0)
        .with_style(Style::filled(Color::black()));

    let mut right = Drawing::new()
        .with_shape(Shape::Rectangle { height: 20 * height, width: 1 })
        .with_xy((20 * width - 1) as f32, 0.0)
        .with_style(Style::filled(Color::black()));

    let mut bottom = Drawing::new()
        .with_shape(Shape::Rectangle { height: 1, width: width * 20 })
        .with_xy(0.0, (20 * height - 1) as f32)
        .with_style(Style::filled(Color::black()));

    let mut left = Drawing::new()
        .with_shape(Shape::Rectangle { height: 20 * height, width: 1 })
        .with_xy(0.0, 0.0)
        .with_style(Style::filled(Color::black()));

    c.display_list.add(top);
    c.display_list.add(right);
    c.display_list.add(bottom);
    c.display_list.add(left);
}


impl MazePhenotype {
    pub fn visualize(&self) {
        let mut canvas = Canvas::new(self.height * 20, self.width * 20);

        add_borders(&mut canvas, self.height, self.width);


        for (x, row) in self.grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if cell.is_waypoint || cell.is_juncture {
                    println!["found point: {}, {}, {}, {}", x, y, cell.is_waypoint, cell.is_juncture];
                    let mut rect = Drawing::new()
                        .with_shape(Shape::Circle {
                            radius: 2,
                        })
                        .with_xy((20 * x + 10) as f32, (20 * y + 10) as f32)
                        .with_style(Style::filled(Color::black()));

                    canvas.display_list.add(rect);
                }
            }
        }

        render::save(
            &canvas,
            "testing/test.svg",
            SvgRenderer::new(),
        ).expect("Failed to save")
    }
}