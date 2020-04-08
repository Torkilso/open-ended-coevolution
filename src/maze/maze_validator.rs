use crate::maze::maze_genotype::PathGene;
use crate::maze::maze_phenotype::MazeCell;
use crate::maze::{Orientation, PathDirection};

#[derive(Debug, Clone)]
pub struct MazeValidator {
    pub width: u32,
    pub height: u32,
    pub first_direction: Orientation,
    pub grid: Vec<Vec<MazeCell>>,
}

impl MazeValidator {
    pub fn new(
        width: u32,
        height: u32,
        first_direction: Orientation,
        path_genes: &Vec<PathGene>,
    ) -> MazeValidator {
        let mut validator = MazeValidator {
            width,
            height,
            first_direction,
            grid: vec![vec![MazeCell::new(); height as usize]; width as usize],
        };
        validator.add_path(path_genes);
        validator
    }

    pub fn validate_new_path(
        width: u32,
        height: u32,
        first_direction: Orientation,
        path_genes: &Vec<PathGene>,
    ) -> bool {
        let mut validator = MazeValidator {
            width,
            height,
            first_direction,
            grid: vec![vec![MazeCell::new(); height as usize]; width as usize],
        };
        validator.add_path(path_genes)
    }

    pub fn get_cell_at(&self, x: u32, y: u32) -> &MazeCell {
        &self.grid[x as usize][y as usize]
    }

    pub fn update_cell_is_juncture(&mut self, x: u32, y: u32, is_juncture: bool) {
        self.grid[x as usize][y as usize].is_juncture = is_juncture;
    }

    pub fn update_cell_is_waypoint(&mut self, x: u32, y: u32, is_waypoint: bool) {
        self.grid[x as usize][y as usize].is_waypoint = is_waypoint;
    }

    pub fn update_cell_path_direction(&mut self, x: u32, y: u32, path_direction: PathDirection) {
        self.grid[x as usize][y as usize].path_direction = path_direction;
    }

    pub fn add_path(&mut self, path_genes: &Vec<PathGene>) -> bool {
        let start_position = PathGene::new(0, self.height - 1);
        if !self.add_waypoint(&start_position, &path_genes[0]) {
            return false;
        }

        for (i, path_gene) in path_genes[0..path_genes.len() - 1].iter().enumerate() {
            let target_point = &path_genes[i + 1];
            if !self.add_waypoint(&path_gene, &target_point) {
                return false;
            }
        }

        let end_position = PathGene::new(self.width - 1, 0);
        if !self.add_waypoint(&path_genes[path_genes.len() - 1], &end_position) {
            return false;
        }

        self.update_cell_path_direction(self.width - 1, 0, PathDirection::South);

        true
    }

    pub fn add_waypoint(&mut self, current_point: &PathGene, target_point: &PathGene) -> bool {
        self.update_cell_is_waypoint(target_point.x, target_point.y, true);

        //println!("current point {} {}", current_point.x, current_point.y);
        //println!("target point {} {}", target_point.x, target_point.y);

        if self.first_direction == Orientation::Vertical {
            if !self.add_vertical_path_segment(current_point.x, current_point.y, target_point.y) {
                return false;
            }
            if !self.add_horizontal_path_segment(target_point.y, current_point.x, target_point.x) {
                return false;
            }
            if current_point.x != target_point.x && current_point.y != target_point.y {
                self.update_cell_is_juncture(current_point.x, target_point.y, true);
            }
        } else if self.first_direction == Orientation::Horizontal {
            if !self.add_horizontal_path_segment(current_point.y, current_point.x, target_point.x) {
                return false;
            }
            if !self.add_vertical_path_segment(target_point.x, current_point.y, target_point.y) {
                return false;
            }
            if current_point.x != target_point.x && current_point.y != target_point.y {
                self.update_cell_is_juncture(target_point.x, current_point.y, true);
            }
        }
        true
    }

    pub fn add_vertical_path_segment(&mut self, from_x: u32, from_y: u32, to_y: u32) -> bool {
        println!(
            "\nAdding vertical path from ({}, {}) to ({}, {})\n",
            from_x, from_y, from_x, to_y
        );

        if from_y <= to_y {
            for y in from_y..to_y + 1 {
                if y == to_y {
                    continue;
                }
                println!(
                    "{} {} cell path direction {:#?} -> North",
                    from_x,
                    y,
                    self.get_cell_at(from_x, y).path_direction,
                );

                if self.get_cell_at(from_x, y).path_direction != PathDirection::None {
                    return false;
                }
                self.update_cell_path_direction(from_x, y, PathDirection::North);
            }
        } else {
            for y in to_y..from_y + 1 {
                if y == to_y {
                    continue;
                }
                println!(
                    "{} {} cell path direction {:#?} -> South",
                    from_x,
                    y,
                    self.get_cell_at(from_x, y).path_direction
                );

                if self.get_cell_at(from_x, y).path_direction != PathDirection::None {
                    return false;
                }
                self.update_cell_path_direction(from_x, y, PathDirection::South);
            }
        }
        true
    }

    pub fn add_horizontal_path_segment(&mut self, from_y: u32, from_x: u32, to_x: u32) -> bool {
        println!(
            "\nAdding horizontal path from ({}, {}) to ({}, {})\n",
            from_x, from_y, to_x, from_y
        );

        if from_x <= to_x {
            for x in from_x..to_x + 1 {
                if x == to_x {
                    continue;
                }

                println!(
                    "{} {} cell path direction {:#?} -> East",
                    x,
                    from_y,
                    self.get_cell_at(x, from_y).path_direction
                );

                if self.get_cell_at(x, from_y).path_direction != PathDirection::None {
                    return false;
                }
                self.update_cell_path_direction(x, from_y, PathDirection::East);
            }
        } else {
            for x in to_x..from_x + 1 {
                if x == to_x {
                    continue;
                }

                println!(
                    "{} {} cell path direction {:#?} -> West",
                    x,
                    from_y,
                    self.get_cell_at(x, from_y).path_direction
                );

                if self.get_cell_at(x, from_y).path_direction != PathDirection::None {
                    return false;
                }
                self.update_cell_path_direction(x, from_y, PathDirection::West);
            }
        }
        true
    }
}
