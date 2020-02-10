extern crate queues;

use math::round;
use queues::*;
use crate::maze_genotype::{PathGene, WallGene};
use crate::general::{Orientation, PathDirection};


#[derive(Debug, Clone)]
pub struct MazeCell {
    pub north_wall: bool,
    pub east_wall: bool,
    pub south_wall: bool,
    pub west_wall: bool,
    pub is_waypoint: bool,
    pub is_juncture: bool,
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

    pub fn print(&self) {
        println!("{:?}", self.grid)
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
            // TODO make sure no wall still blocks openings
        }
    }

    pub fn update_cell_wall_east(&mut self, x: u32, y: u32, wall: bool) {
        self.grid[x as usize][y as usize].east_wall = wall;
    }

    pub fn update_cell_wall_south(&mut self, x: u32, y: u32, wall: bool) {
        self.grid[x as usize][y as usize].south_wall = wall;
    }

    pub fn update_cell_wall_west(&mut self, x: u32, y: u32, wall: bool) {
        self.grid[x as usize][y as usize].west_wall = wall;
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
        let orientation = target_point.orientation;
        self.update_cell_is_waypoint(target_point.x, target_point.y, true);

        if orientation == Orientation::Horizontal {
            self.horizontal_path_reroute(&current_point, &target_point);
            self.add_vertical_path_segment(&current_point, &target_point);
            self.add_horizontal_path_segment(&current_point, &target_point);

            if current_point.x != target_point.x && current_point.y != target_point.y {
                self.update_cell_is_juncture(current_point.x, target_point.y, true);
            }
        } else {
            self.vertical_path_reroute(&current_point, &target_point);
            self.add_horizontal_path_segment(&current_point, &target_point);
            self.add_vertical_path_segment(&current_point, &target_point);

            if current_point.x != target_point.x && current_point.y != target_point.y {
                self.update_cell_is_juncture(target_point.x, current_point.y, true);
            }
        }
    }

    pub fn add_vertical_path_segment(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if current_point.y <= end_point.y {
            for y in current_point.y..end_point.y {
                self.update_cell_path_direction(end_point.x, y, PathDirection::South);
            }
        } else {
            for y in end_point.y..current_point.y {
                self.update_cell_path_direction(end_point.x, y, PathDirection::North);
            }
        }
    }

    pub fn add_horizontal_path_segment(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if current_point.x <= end_point.x {
            for x in current_point.x..end_point.x {
                self.update_cell_path_direction(x, current_point.y, PathDirection::East);
            }
        } else {
            for x in end_point.x..current_point.x {
                self.update_cell_path_direction(x, current_point.y, PathDirection::West);
            }
        }
    }

    pub fn horizontal_path_reroute(&mut self, current_point: &PathGene, end_point: &PathGene) {
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
    }

    pub fn add_walls(&mut self, wall_genes: &Vec<WallGene>) {
        let mut current_wall_gene = 0;
        let mut loop_iteration = 0;

        self.enclose_adjacent_path_segments();
        let subdivisions = self.subdivide_maze();

        println!("{:#?}", subdivisions);

        /*for subdivision in subdivisions {
            let mut subdivision_queue: Queue<MazeSubdivision> = queue![];
            if subdivision.height > 1 || subdivision.width > 1 {
                loop_iteration += 1;
                current_wall_gene = loop_iteration % wall_genes.len();

                self.mark_subdivision_boundaries(&subdivision);
                self.insert_partition_opening(&subdivision, &wall_genes[current_wall_gene]);

                subdivision_queue.add(subdivision);

                while subdivision_queue.size() > 0 {
                    let current_subdivision = subdivision_queue.remove().unwrap_or(panic! {});

                    loop_iteration += 1;
                    current_wall_gene = loop_iteration % wall_genes.len();

                    let (child_1, child_2) = self.subdivide_subdivision(
                        &current_subdivision,
                        &wall_genes[current_wall_gene],
                    );

                    if child_1.width > 1 && child_1.height > 1 {
                        subdivision_queue.add(child_1);
                    }

                    if child_2.width > 1 && child_2.height > 1 {
                        subdivision_queue.add(child_2);
                    }
                }
            } else {
                self.mark_subdivision_boundaries(&subdivision)
            }
        }*/
    }

    pub fn enclose_adjacent_path_segments(&mut self) {
        let mut current_x = 0;
        let mut current_y = 0;

        while current_x < self.width && current_y < self.height {
            if self.get_cell_at(current_x, current_y).path_direction != PathDirection::North
                && current_y > 0
                && self.get_cell_at(current_x, current_y - 1).path_direction == PathDirection::None
            {
                self.update_cell_wall_north(current_x, current_y, true);
            }

            if self.get_cell_at(current_x, current_y).path_direction != PathDirection::South
                && current_y + 1 < self.height
                && self.get_cell_at(current_x, current_y + 1).path_direction == PathDirection::None
            {
                self.update_cell_wall_south(current_x, current_y, true);
            }

            if self.get_cell_at(current_x, current_y).path_direction != PathDirection::West
                && current_x > 0
                && self.get_cell_at(current_x - 1, current_y).path_direction == PathDirection::None
            {
                self.update_cell_wall_west(current_x, current_y, true);
            }

            if self.get_cell_at(current_x, current_y).path_direction != PathDirection::East
                && current_x + 1 < self.width
                && self.get_cell_at(current_x + 1, current_y).path_direction == PathDirection::None
            {
                self.update_cell_wall_east(current_x, current_y, true);
            }

            let current_cell = self.get_cell_at(current_x, current_y);

            if current_cell.path_direction == PathDirection::North {
                current_y -= 1;
            } else if current_cell.path_direction == PathDirection::East {
                current_x += 1;
            } else if current_cell.path_direction == PathDirection::South {
                current_y += 1;
            } else if current_cell.path_direction == PathDirection::West {
                current_x -= 1;
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
                    println!("subdivisions found: {}\n\n", subdivisions.len());
                    let mut start_x = x;
                    let mut start_y = y;

                    let mut end_x = start_x;
                    let mut end_y = start_y;

                    while end_x < self.width - 1
                        && self.get_cell_at(end_x + 1, end_y).path_direction == PathDirection::None
                    {
                        end_x += 1;
                    }
                    println!("end_x {}", end_x);
                    println!("finding end_y");

                    let mut blockade_found = false;

                    for y_search in start_y..self.height {
                        for x_search in start_x..end_x + 1 {
                            println!("{}, {}", x_search, y_search);
                            if self.get_cell_at(x_search, y_search).path_direction != PathDirection::None {
                                println!("breaking at {}, {}", x_search, y_search);
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

                    println!("end values: {}, {}", end_x, end_y);

                    subdivisions.push(MazeSubdivision::new(
                        start_x,
                        start_y,
                        end_x,
                        end_y,
                        end_x - start_x + 1,
                        end_y - start_y + 1,
                    ));

                    self.update_cells_in_subdivision(start_x, start_y, end_x, end_y);
                }
            }
        }
        return subdivisions;
    }

    pub fn mark_subdivision_boundaries(&mut self, subdivision: &MazeSubdivision) {
        for x in subdivision.start_x..subdivision.end_x {
            self.update_cell_wall_north(x, subdivision.start_y, true);
            self.update_cell_wall_south(x, subdivision.end_y, true);
        }

        for y in subdivision.start_y..subdivision.end_y {
            self.update_cell_wall_west(subdivision.start_x, y, true);
            self.update_cell_wall_east(subdivision.end_x, y, true);
        }

        if subdivision.width == 1 || subdivision.height == 1 {
            if subdivision.start_x == 0 {
                self.update_cell_wall_east(subdivision.end_x, subdivision.start_y, false);
            } else {
                self.update_cell_wall_west(subdivision.start_x, subdivision.start_y, false);
            }
        }
    }

    pub fn insert_partition_opening(
        &mut self,
        subdivision: &MazeSubdivision,
        wall_gene: &WallGene,
    ) {
        if wall_gene.orientation == Orientation::Horizontal {
            let y = round::floor(
                (subdivision.start_y as f32 + subdivision.height as f32 * wall_gene.wall_position)
                    as f64,
                0,
            );

            if subdivision.start_x > 0 {
                self.update_cell_wall_west(subdivision.start_x, y as u32, true);
            } else {
                self.update_cell_wall_east(subdivision.end_x, y as u32, true);
            }
        } else {
            let y = round::floor(
                (subdivision.start_y as f32
                    + subdivision.height as f32 * wall_gene.passage_position)
                    as f64,
                0,
            );

            if subdivision.start_x > 0 {
                self.update_cell_wall_west(subdivision.start_x, y as u32, true);
            } else {
                self.update_cell_wall_east(subdivision.end_x, y as u32, true);
            }
        }
    }

    pub fn subdivide_subdivision(
        &mut self,
        subdivision: &MazeSubdivision,
        wall_gene: &WallGene,
    ) -> (MazeSubdivision, MazeSubdivision) {
        return if wall_gene.orientation == Orientation::Horizontal {
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

            for x in 0..subdivision.width {
                if x != passage_location_x as u32 {
                    self.update_cell_wall_south(subdivision.start_x + x, wall_location_y, true);
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
                start_x: subdivision.start_x,
                start_y: wall_location_y + 1,
                end_x: subdivision.end_x,
                end_y: subdivision.end_y,
                width: subdivision.width,
                height: subdivision.end_y - wall_location_y + 1 + 1,
            };
            (child_1, child_2)
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

            for y in 0..subdivision.height {
                if y != passage_location_y as u32 {
                    self.update_cell_wall_east(wall_location_x, subdivision.start_y + y, true);
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
                start_x: wall_location_x,
                start_y: subdivision.start_y,
                end_x: subdivision.end_x,
                end_y: subdivision.end_y,
                width: subdivision.end_x - wall_location_x + 1,
                height: subdivision.height,
            };
            (child_1, child_2)
        };
    }
}

