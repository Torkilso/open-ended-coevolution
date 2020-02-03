use crate::maze::Orientation::Horizontal;
use draw::*;
use crate::MazeMutationOptions;

#[derive(Debug, Clone)]
pub struct WallGene {
    position: f32,
    passage: f32,
}

impl WallGene {
    pub fn new(position: f32, passage: f32) -> WallGene {
        WallGene {
            position,
            passage,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct PathGene {
    x: usize,
    y: usize,
    orientation: Orientation,
}

impl PathGene {
    pub fn new(x: usize, y: usize, orientation: Orientation) -> PathGene {
        PathGene {
            x,
            y,
            orientation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MazeGenome {
    width: usize,
    height: usize,
    path_genes: Vec<PathGene>,
    wall_genes: Vec<WallGene>,
}

impl MazeGenome {
    pub fn new(width: usize, height: usize, path_genes: Vec<PathGene>, wall_genes: Vec<WallGene>) -> MazeGenome {
        MazeGenome {
            width,
            height,
            path_genes,
            wall_genes,
        }
    }

    pub fn get_path_genes(&self) -> &Vec<PathGene> {
        &self.path_genes
    }

    pub fn get_wall_genes(&self) -> &Vec<WallGene> {
        &self.wall_genes
    }

    pub fn mutate(&self, options: &MazeMutationOptions) {
        //
    }

    pub fn to_phenotype(&self) -> MazePhenotype {
        let start_location = (0, 0);
        let end_location = (self.width, self.height);

        let mut phenotype = MazePhenotype::new(self.width, self.height);
        phenotype.add_path(self.get_path_genes());
        phenotype.add_walls(self.get_wall_genes());

        phenotype
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PathDirection {
    North,
    East,
    South,
    West,
    None,
}

#[derive(Debug, Clone)]
pub struct MazeCell {
    north_wall: bool,
    east_wall: bool,
    south_wall: bool,
    west_wall: bool,
    is_waypoint: bool,
    is_juncture: bool,
    path_direction: PathDirection,
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct MazePhenotype {
    width: usize,
    height: usize,
    grid: Vec<Vec<MazeCell>>,
}

impl MazePhenotype {
    pub fn new(width: usize, height: usize) -> MazePhenotype {
        MazePhenotype {
            width,
            height,
            grid: vec![vec![MazeCell::new(); height]; width],
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.grid)
    }

    pub fn add_path(&mut self, path_genes: &Vec<PathGene>) {
        let first = 0;

        for (i, path_gene) in path_genes.iter().enumerate() {
            let current_point: PathGene;
            let target_point: PathGene;

            if i == 0 {
                current_point = PathGene::new(0, 0, Orientation::Vertical)
            } else {
                current_point = path_genes[i - 1].clone()
            }

            if i == path_genes.len() - 1 {
                target_point = PathGene::new(self.width - 1, self.height - 1, Orientation::Vertical)
            } else {
                target_point = path_genes[i].clone()
            }

            let orientation = target_point.orientation;
            self.grid[target_point.x][target_point.y].is_waypoint = true;

            if orientation == Orientation::Horizontal {
                self.horizontal_path_reroute(&current_point, &target_point);
                self.add_vertical_path_segment(&current_point, &target_point);
                self.add_horizontal_path_segment(&current_point, &target_point);

                if current_point.x != target_point.x && current_point.y != target_point.y {
                    self.grid[current_point.x][target_point.y].is_juncture = true;
                }
            } else {
                self.vertical_path_reroute(&current_point, &target_point);
                self.add_horizontal_path_segment(&current_point, &target_point);
                self.add_vertical_path_segment(&current_point, &target_point);

                if current_point.x != target_point.x && current_point.y != target_point.y {
                    self.grid[target_point.x][current_point.y].is_juncture = true;
                }
            }
        }
    }

    pub fn add_vertical_path_segment(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if current_point.y <= end_point.y {
            for y in current_point.y..end_point.y {
                self.grid[current_point.x][y].path_direction = PathDirection::South;
            }
        } else {
            for y in end_point.y..current_point.y {
                self.grid[current_point.x][y].path_direction = PathDirection::North;
            }
        }
    }

    pub fn add_horizontal_path_segment(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if current_point.x <= end_point.x {
            for x in current_point.x..end_point.x {
                self.grid[x][current_point.y].path_direction = PathDirection::East;
            }
        } else {
            for x in end_point.x..current_point.x {
                self.grid[x][current_point.y].path_direction = PathDirection::West;
            }
        }
    }

    pub fn horizontal_path_reroute(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if end_point.y < current_point.y && end_point.x > current_point.x {
            let mut current_y = current_point.y;
            if self.grid[current_point.x + 1][current_y].path_direction == PathDirection::None {
                self.grid[current_point.x][current_y].path_direction = PathDirection::South;
                self.grid[current_point.x][current_y + 1].is_juncture = true;
                current_y += 1;
            }

            let rightmost_waypoint_x = if current_point.x > end_point.x { current_point.x } else { end_point.x };

            for x in current_point.x..rightmost_waypoint_x {
                self.grid[x][current_y].path_direction = PathDirection::South;
            }
            self.grid[current_point.x][current_y].is_juncture = true;
        }
    }

    pub fn vertical_path_reroute(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if end_point.x < current_point.x && end_point.y > current_point.y {
            let mut current_x = current_point.x;
            if self.grid[current_x][current_point.y + 1].path_direction == PathDirection::None {
                self.grid[current_x][current_point.y].path_direction = PathDirection::East;
                self.grid[current_x + 1][current_point.y].is_juncture = true;
                current_x += 1;
            }

            let lowest_waypoint_y = if current_point.x > end_point.x { current_point.x } else { end_point.x };

            for y in current_point.y..lowest_waypoint_y {
                self.grid[current_x][y].path_direction = PathDirection::South;
            }
            self.grid[current_x][current_point.y].is_juncture = true;
        }
    }

    pub fn add_walls(&self, wall_genes: &Vec<WallGene>) {}

    pub fn enclose_adjacent_path_segments(&self) {}

    pub fn subdivide_maze(&self) {}

    pub fn mark_partition_boundaries(&self) {}

    pub fn insert_partition_opening(&self) {}

    pub fn subdivide_partition(&self) {}

    pub fn visualize(&self) {}
}

/*pub fn generate_random_maze_genome() -> MazeGenome {
    MazeGenome::new()
}

pub fn generate_random_mazes(amount: i32) -> Vec<MazeGenome> {
    let new_maze = generate_random_maze_genome();
    vec![new_maze]
}*/

