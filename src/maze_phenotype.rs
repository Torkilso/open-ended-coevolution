extern crate queues;

use math::round;
use queues::*;
use crate::maze_genotype::{PathGene, WallGene};
use crate::general::{Orientation, PathDirection, OpeningLocation};
use std::borrow::{Borrow, BorrowMut};


#[derive(Debug, Clone)]
pub struct MazeCell {
    pub north_wall: bool,
    pub east_wall: bool,
    pub south_wall: bool,
    pub west_wall: bool,
    pub is_waypoint: bool,
    pub is_juncture: bool,
    pub passage_direction: PathDirection,
    pub path_direction: PathDirection,
    pub in_subdivision: bool,
}

impl MazeCell {
    pub fn new() -> MazeCell {
        MazeCell {
            north_wall: false,
            east_wall: false,
            south_wall: false,
            west_wall: false,
            is_waypoint: false,
            is_juncture: false,
            passage_direction: PathDirection::None,
            path_direction: PathDirection::None,
            in_subdivision: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MazeSubdivision {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
    width: u32,
    height: u32,
}

impl MazeSubdivision {
    pub fn new(
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        width: u32,
        height: u32,
    ) -> MazeSubdivision {
        MazeSubdivision {
            start_x,
            start_y,
            end_x,
            end_y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MazePhenotype {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Vec<MazeCell>>,
}

impl MazePhenotype {
    pub fn new(width: u32, height: u32, path_genes: &Vec<PathGene>, wall_genes: &Vec<WallGene>) -> MazePhenotype {
        let mut phenotype = MazePhenotype {
            width,
            height,
            grid: vec![vec![MazeCell::new(); height as usize]; width as usize],
        };
        phenotype.add_path(path_genes);
        phenotype.add_walls(wall_genes);

        phenotype
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

    pub fn update_cell_wall_north(&mut self, x: u32, y: u32, wall: bool) {
        self.grid[x as usize][y as usize].north_wall = wall;
        if y > 0 {
            self.grid[x as usize][y as usize - 1].south_wall = wall;
        }
    }

    pub fn update_cell_wall_east(&mut self, x: u32, y: u32, wall: bool) {
        self.grid[x as usize][y as usize].east_wall = wall;
        if x < self.width - 1 {
            self.grid[x as usize + 1][y as usize].west_wall = wall;
        }
    }

    pub fn update_cell_wall_south(&mut self, x: u32, y: u32, wall: bool) {
        self.grid[x as usize][y as usize].south_wall = wall;
        if y < self.height - 1 {
            self.grid[x as usize][y as usize + 1].north_wall = wall;
        }
    }

    pub fn update_cell_wall_west(&mut self, x: u32, y: u32, wall: bool) {
        self.grid[x as usize][y as usize].west_wall = wall;
        if x > 0 {
            self.grid[x as usize - 1][y as usize].east_wall = wall;
        }
    }

    pub fn update_cell_passage_direction(&mut self, x: u32, y: u32, passage_direction: PathDirection) {
        self.grid[x as usize][y as usize].passage_direction = passage_direction;
    }

    pub fn update_cell_in_subdivision(&mut self, x: u32, y: u32, value: bool) {
        self.grid[x as usize][y as usize].in_subdivision = value;
    }

    pub fn update_cells_in_subdivision(&mut self, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
        for x in start_x..end_x + 1 {
            for y in start_y..end_y + 1 {
                self.update_cell_in_subdivision(x, y, true);
            }
        }
    }

    pub fn add_path(&mut self, path_genes: &Vec<PathGene>) {
        let start_position = PathGene::new(0, 0, Orientation::Vertical);
        self.add_waypoint(&start_position, &path_genes[0]);

        for (i, path_gene) in path_genes[0..path_genes.len() - 1].iter().enumerate() {
            let target_point = &path_genes[i + 1];
            self.add_waypoint(&path_gene, &target_point);
        }

        let end_position = PathGene::new(
            self.width - 1,
            self.height - 1,
            path_genes[path_genes.len() - 1].orientation,
        );
        self.add_waypoint(&path_genes[path_genes.len() - 1], &end_position);
    }

    pub fn add_waypoint(&mut self, current_point: &PathGene, target_point: &PathGene) {
        self.update_cell_is_waypoint(target_point.x, target_point.y, true);

        if target_point.orientation == Orientation::Horizontal {
            //self.horizontal_path_reroute(&current_point, &target_point);
            self.add_vertical_path_segment(&current_point, &target_point);
            self.add_horizontal_path_segment(&current_point, &target_point);

            if current_point.x != target_point.x && current_point.y != target_point.y {
                self.update_cell_is_juncture(current_point.x, target_point.y, true);
            }
        } else {
            //self.vertical_path_reroute(&current_point, &target_point);
            self.add_horizontal_path_segment(&current_point, &target_point);
            self.add_vertical_path_segment(&current_point, &target_point);

            if current_point.x != target_point.x && current_point.y != target_point.y {
                self.update_cell_is_juncture(target_point.x, current_point.y, true);
            }
        }
    }

    pub fn add_vertical_path_segment(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if current_point.y <= end_point.y {
            for y in current_point.y..end_point.y + 1 {
                self.update_cell_path_direction(end_point.x, y, PathDirection::South);
            }
        } else {
            for y in end_point.y..current_point.y + 1 {
                self.update_cell_path_direction(end_point.x, y, PathDirection::North);
            }
        }
    }

    pub fn add_horizontal_path_segment(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if current_point.x <= end_point.x {
            for x in current_point.x..end_point.x + 1 {
                self.update_cell_path_direction(x, current_point.y, PathDirection::East);
            }
        } else {
            for x in end_point.x..current_point.x + 1 {
                self.update_cell_path_direction(x, current_point.y, PathDirection::West);
            }
        }
    }

    /*pub fn horizontal_path_reroute(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if end_point.y < current_point.y && end_point.x > current_point.x {
            let mut current_y = current_point.y;

            if self
                .get_cell_at(current_point.x + 1, current_y)
                .path_direction
                == PathDirection::None
            {
                self.update_cell_path_direction(current_point.x, current_y, PathDirection::South);
                self.update_cell_is_juncture(current_point.x, current_y, true);
                current_y += 1;
            }

            let rightmost_waypoint_x = if current_point.x > end_point.x {
                current_point.x
            } else {
                end_point.x
            };

            for x in current_point.x..rightmost_waypoint_x {
                self.update_cell_path_direction(x, current_y, PathDirection::South);
            }
            self.update_cell_is_juncture(current_point.x, current_y, true);
        }
    }

    pub fn vertical_path_reroute(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if end_point.x < current_point.x && end_point.y > current_point.y {
            let mut current_x = current_point.x;

            if self
                .get_cell_at(current_x, current_point.y + 1)
                .path_direction
                == PathDirection::None
            {
                self.update_cell_path_direction(current_x, current_point.y, PathDirection::East);
                self.update_cell_is_juncture(current_x + 1, current_point.y, true);
                current_x += 1;
            }
            let lowest_waypoint_y = if current_point.x > end_point.x {
                current_point.x
            } else {
                end_point.x
            };

            for y in current_point.y..lowest_waypoint_y {
                self.update_cell_path_direction(current_x, y, PathDirection::South);
            }
            self.update_cell_is_juncture(current_x, current_point.y, true);
        }
    }*/

    pub fn add_walls(&mut self, wall_genes: &Vec<WallGene>) {
        self.enclose_adjacent_path_segments();
        let subdivisions = self.subdivide_maze();

        for subdivision in subdivisions.iter() {
            self.mark_subdivision_boundaries(&subdivision);
        }

        for (i, current_subdivision) in subdivisions.iter().enumerate() {
            let current_wall_gene = i % wall_genes.len();
            self.insert_partition_opening(&current_subdivision, &wall_genes[current_wall_gene]);

            self.subdivide_subdivision(
                &current_subdivision,
                current_wall_gene,
                &wall_genes,
            );
        }
    }

    pub fn enclose_adjacent_path_segments(&mut self) {
        let mut x = 0;
        let mut y = 0;

        let mut previous_x: u32 = x;
        let mut previous_y: u32 = y;

        while x != self.width && y != self.height {
            let cell = self.get_cell_at(x, y).clone();

            if cell.path_direction == PathDirection::North || cell.path_direction == PathDirection::South {
                if self.get_cell_at(previous_x, previous_y).path_direction != PathDirection::East {
                    self.update_cell_wall_west(x, y, true);
                }
                if self.get_cell_at(previous_x, previous_y).path_direction != PathDirection::West {
                    self.update_cell_wall_east(x, y, true);
                }
            }

            if cell.path_direction == PathDirection::West || cell.path_direction == PathDirection::East {
                if self.get_cell_at(previous_x, previous_y).path_direction != PathDirection::South {
                    self.update_cell_wall_north(x, y, true);
                }
                if self.get_cell_at(previous_x, previous_y).path_direction != PathDirection::North {
                    self.update_cell_wall_south(x, y, true);
                }
            }

            previous_x = x;
            previous_y = y;

            if cell.path_direction == PathDirection::North {
                y -= 1;
            } else if cell.path_direction == PathDirection::East {
                x += 1;
            } else if cell.path_direction == PathDirection::South {
                y += 1;
            } else if cell.path_direction == PathDirection::West {
                x -= 1;
            } else {
                break;
            }
        }
    }

    pub fn subdivide_maze(&mut self) -> Vec<MazeSubdivision> {
        let mut subdivisions: Vec<MazeSubdivision> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_cell_at(x, y).path_direction == PathDirection::None &&
                    !self.get_cell_at(x, y).is_waypoint &&
                    !self.get_cell_at(x, y).in_subdivision
                {
                    let start_x = x;
                    let start_y = y;

                    let mut end_x = start_x;
                    let mut end_y = start_y;

                    while end_x < self.width - 1
                        && self.get_cell_at(end_x + 1, end_y).path_direction == PathDirection::None
                        && !self.get_cell_at(end_x + 1, end_y).in_subdivision
                    {
                        end_x += 1;
                    }
                    let mut blockade_found = false;

                    for y_search in start_y..self.height {
                        for x_search in start_x..end_x + 1 {
                            if self.get_cell_at(x_search, y_search).path_direction != PathDirection::None && !self.get_cell_at(x_search, y_search).in_subdivision {
                                blockade_found = true;
                                break;
                            }
                        }

                        if blockade_found {
                            end_y = y_search - 1;
                            break;
                        }
                    }

                    if !blockade_found {
                        end_y = self.height - 1;
                    }

                    self.update_cells_in_subdivision(start_x, start_y, end_x, end_y);

                    subdivisions.push(MazeSubdivision::new(
                        start_x,
                        start_y,
                        end_x,
                        end_y,
                        end_x - start_x + 1,
                        end_y - start_y + 1,
                    ));
                }
            }
        }
        return subdivisions;
    }

    pub fn mark_subdivision_boundaries(&mut self, subdivision: &MazeSubdivision) {
        //println!("marking boundaries");

        for x in subdivision.start_x..subdivision.end_x + 1 {
            self.update_cell_wall_north(x, subdivision.start_y, true);
            self.update_cell_wall_south(x, subdivision.end_y, true);
        }

        for y in subdivision.start_y..subdivision.end_y + 1 {
            self.update_cell_wall_west(subdivision.start_x, y, true);
            self.update_cell_wall_east(subdivision.end_x, y, true);
        }

        /*if subdivision.width == 1 || subdivision.height == 1 {
            if subdivision.start_x == 0 {
                self.update_cell_wall_east(subdivision.end_x, subdivision.start_y, false);
            } else {
                self.update_cell_wall_west(subdivision.start_x, subdivision.start_y, false);
            }
        }*/
    }

    pub fn insert_partition_opening(
        &mut self,
        subdivision: &MazeSubdivision,
        wall_gene: &WallGene,
    ) {

        // find side with wall against path
        let mut north_wall = 0;
        let mut south_wall = 0;
        let mut west_wall = 0;
        let mut east_wall = 0;

        if subdivision.start_y > 0 {
            for x in subdivision.start_x..subdivision.end_x + 1 {
                if self.get_cell_at(x, subdivision.start_y - 1).path_direction != PathDirection::None {
                    north_wall += 1;
                }
            }
        }
        if subdivision.end_y < self.height - 1 {
            for x in subdivision.start_x..subdivision.end_x + 1 {
                if self.get_cell_at(x, subdivision.end_y + 1).path_direction != PathDirection::None {
                    south_wall += 1;
                }
            }
        }
        if subdivision.start_x > 0 {
            for y in subdivision.start_y..subdivision.end_y + 1 {
                if self.get_cell_at(subdivision.start_x - 1, y).path_direction != PathDirection::None {
                    west_wall += 1;
                }
            }
        }
        if subdivision.end_x < self.width - 1 {
            for y in subdivision.start_y..subdivision.end_y + 1 {
                if self.get_cell_at(subdivision.end_x + 1, y).path_direction != PathDirection::None {
                    east_wall += 1;
                }
            }
        }

        let x = round::floor(
            (subdivision.start_x as f32
                + subdivision.width as f32 * wall_gene.passage_position)
                as f64,
            0,
        ) as u32;

        let y = round::floor(
            (subdivision.start_y as f32 + subdivision.height as f32 * wall_gene.passage_position)
                as f64,
            0,
        ) as u32;

        if wall_gene.opening_location == OpeningLocation::North {
            if north_wall > 0 {
                for current_x in 0..subdivision.width {
                    let x_to_use = subdivision.start_x + (x + current_x) % subdivision.width;
                    if self.get_cell_at(x_to_use, subdivision.start_y - 1).path_direction != PathDirection::None {
                        self.update_cell_wall_north(x_to_use, subdivision.start_y, false);
                        break;
                    }
                }
            } else if south_wall > 0 {
                for current_x in 0..subdivision.width {
                    let x_to_use = subdivision.start_x + (x + current_x) % subdivision.width;
                    if self.get_cell_at(x_to_use, subdivision.end_y + 1).path_direction != PathDirection::None {
                        self.update_cell_wall_south(x_to_use, subdivision.end_y, false);
                        break;
                    }
                }
            } else if west_wall > 0 && west_wall >= east_wall {
                for current_y in 0..subdivision.height {
                    let y_to_use = subdivision.start_y + (y + current_y) % subdivision.height;
                    if self.get_cell_at(subdivision.start_x - 1, y_to_use).path_direction != PathDirection::None {
                        self.update_cell_wall_west(subdivision.start_x, y_to_use, false);
                        break;
                    }
                }
            } else {
                for current_y in 0..subdivision.height {
                    let y_to_use = subdivision.start_y + (y + current_y) % subdivision.height;
                    if self.get_cell_at(subdivision.end_x + 1, y_to_use).path_direction != PathDirection::None {
                        self.update_cell_wall_east(subdivision.end_x, y_to_use, false);
                        break;
                    }
                }
            }
        }
        //println!("{} {} {} {} {:#?}", north_wall, east_wall, west_wall, south_wall, wall_gene.opening_location);

        if wall_gene.opening_location == OpeningLocation::East {
            if east_wall > 0 {
                for current_y in 0..subdivision.height {
                    let y_to_use = subdivision.start_y + (y + current_y) % subdivision.height;
                    if self.get_cell_at(subdivision.end_x + 1, y_to_use).path_direction != PathDirection::None {
                        self.update_cell_wall_east(subdivision.end_x, y_to_use, false);
                        break;
                    }
                }
            } else if west_wall > 0 {
                for current_y in 0..subdivision.height {
                    let y_to_use = subdivision.start_y + (y + current_y) % subdivision.height;
                    if self.get_cell_at(subdivision.start_x - 1, y_to_use).path_direction != PathDirection::None {
                        self.update_cell_wall_west(subdivision.start_x, y_to_use, false);
                        break;
                    }
                }
            } else if north_wall > 0 && north_wall >= south_wall {
                for current_x in 0..subdivision.width {
                    let x_to_use = subdivision.start_x + (x + current_x) % subdivision.width;
                    if self.get_cell_at(x_to_use, subdivision.start_y - 1).path_direction != PathDirection::None {
                        self.update_cell_wall_north(x_to_use, subdivision.start_y, false);
                        break;
                    }
                }
            } else if south_wall > 0 {
                for current_x in 0..subdivision.width {
                    let x_to_use = subdivision.start_x + (x + current_x) % subdivision.width;
                    if self.get_cell_at(x_to_use, subdivision.end_y + 1).path_direction != PathDirection::None {
                        self.update_cell_wall_south(x_to_use, subdivision.end_y, false);
                        break;
                    }
                }
            }
        }
    }

    pub fn subdivide_subdivision(
        &mut self,
        subdivision: &MazeSubdivision,
        wall_gene_index: usize,
        wall_genes: &Vec<WallGene>,
    ) {
        if subdivision.width == 1 || subdivision.height == 1 {
            return;
        }

        let wall_gene = &wall_genes[wall_gene_index];
        let new_wall_gene_index = (wall_gene_index + 1) % wall_genes.len();

        if wall_gene.orientation == Orientation::Horizontal {
            let wall_location_y = round::floor(
                (subdivision.start_y as f32 + subdivision.height as f32 * wall_gene.wall_position)
                    as f64,
                0,
            ) as u32;

            let passage_location_x = round::floor(
                (subdivision.start_x as f32 + subdivision.width as f32 * wall_gene.passage_position)
                    as f64,
                0,
            );

            for x in subdivision.start_x..subdivision.end_x + 1 {
                if x == passage_location_x as u32 {
                    self.update_cell_wall_south(x, wall_location_y, false);
                } else {
                    self.update_cell_wall_south(x, wall_location_y, true);
                }
            }

            let child_1 = MazeSubdivision {
                start_x: subdivision.start_x,
                start_y: subdivision.start_y,
                end_x: subdivision.end_x,
                end_y: wall_location_y,
                width: subdivision.width,
                height: wall_location_y - subdivision.start_y + 1,
            };

            let child_2 = MazeSubdivision {
                start_x: child_1.start_x,
                start_y: wall_location_y + 1,
                end_x: child_1.end_x,
                end_y: subdivision.end_y,
                width: subdivision.width,
                height: subdivision.height - child_1.height,
            };

            self.subdivide_subdivision(&child_1, new_wall_gene_index, wall_genes);
            self.subdivide_subdivision(&child_2, new_wall_gene_index, wall_genes);
        } else {
            let wall_location_x = round::floor(
                (subdivision.start_x as f32 + subdivision.width as f32 * wall_gene.wall_position)
                    as f64,
                0,
            ) as u32;

            let passage_location_y = round::floor(
                (subdivision.start_y as f32
                    + subdivision.height as f32 * wall_gene.passage_position)
                    as f64,
                0,
            );

            for y in subdivision.start_y..subdivision.end_y + 1 {
                if y == passage_location_y as u32 {
                    self.update_cell_wall_east(wall_location_x, y, false);
                } else {
                    self.update_cell_wall_east(wall_location_x, y, true);
                }
            }

            let child_1 = MazeSubdivision {
                start_x: subdivision.start_x,
                start_y: subdivision.start_y,
                end_x: wall_location_x,
                end_y: subdivision.end_y,
                width: wall_location_x - subdivision.start_x + 1,
                height: subdivision.height,
            };

            let child_2 = MazeSubdivision {
                start_x: child_1.end_x + 1,
                start_y: subdivision.start_y,
                end_x: subdivision.end_x,
                end_y: subdivision.end_y,
                width: subdivision.width - child_1.width,
                height: subdivision.height,
            };

            self.subdivide_subdivision(&child_1, new_wall_gene_index, wall_genes);
            self.subdivide_subdivision(&child_2, new_wall_gene_index, wall_genes);
        };
    }
}

