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
    x: u32,
    y: u32,
    orientation: Orientation,
}

impl PathGene {
    pub fn new(x: u32, y: u32, orientation: Orientation) -> PathGene {
        PathGene {
            x,
            y,
            orientation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MazeGenome {
    width: u32,
    height: u32,
    path_genes: Vec<PathGene>,
    wall_genes: Vec<WallGene>,
}

impl MazeGenome {
    pub fn new(width: u32, height: u32, path_genes: Vec<PathGene>, wall_genes: Vec<WallGene>) -> MazeGenome {
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
    pub north_wall: bool,
    pub east_wall: bool,
    pub south_wall: bool,
    pub west_wall: bool,
    pub is_waypoint: bool,
    pub is_juncture: bool,
    pub path_direction: PathDirection,
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
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Vec<MazeCell>>,
}

impl MazePhenotype {
    pub fn new(width: u32, height: u32) -> MazePhenotype {
        MazePhenotype {
            width,
            height,
            grid: vec![vec![MazeCell::new(); height as usize]; width as usize],
        }
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


    pub fn add_path(&mut self, path_genes: &Vec<PathGene>) {
        let start_position = PathGene::new(0, 0, Orientation::Vertical);
        self.add_waypoint(&start_position, &path_genes[0]);


        for (i, path_gene) in path_genes[0..path_genes.len() - 1].iter().enumerate() {
            let target_point = &path_genes[i + 1];
            self.add_waypoint(&path_gene, &target_point);
        }

        let end_position = PathGene::new(self.width - 1, self.height - 1, path_genes[path_genes.len() - 1].orientation);
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
                self.update_cell_path_direction(current_point.x, y, PathDirection::South);
            }
        } else {
            for y in end_point.y..current_point.y {
                self.update_cell_path_direction(current_point.x, y, PathDirection::North);
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

            if self.get_cell_at(current_point.x + 1, current_y).path_direction == PathDirection::None {
                self.update_cell_path_direction(current_point.x, current_y, PathDirection::South);
                self.update_cell_is_juncture(current_point.x, current_y, true);
                current_y += 1;
            }

            let rightmost_waypoint_x = if current_point.x > end_point.x { current_point.x } else { end_point.x };

            for x in current_point.x..rightmost_waypoint_x {
                self.update_cell_path_direction(x, current_y, PathDirection::South);
            }
            self.update_cell_is_juncture(current_point.x, current_y, true);
        }
    }

    pub fn vertical_path_reroute(&mut self, current_point: &PathGene, end_point: &PathGene) {
        if end_point.x < current_point.x && end_point.y > current_point.y {
            let mut current_x = current_point.x;

            if self.get_cell_at(current_x, current_point.y + 1).path_direction == PathDirection::None {
                self.update_cell_path_direction(current_x, current_point.y, PathDirection::East);
                self.update_cell_is_juncture(current_x + 1, current_point.y, true);
                current_x += 1;
            }
            let lowest_waypoint_y = if current_point.x > end_point.x { current_point.x } else { end_point.x };

            for y in current_point.y..lowest_waypoint_y {
                self.update_cell_path_direction(current_x, y, PathDirection::South);
            }
            self.update_cell_is_juncture(current_x, current_point.y, true);
        }
    }

    pub fn add_walls(&self, wall_genes: &Vec<WallGene>) {}

    pub fn enclose_adjacent_path_segments(&self) {}

    pub fn subdivide_maze(&self) {}

    pub fn mark_partition_boundaries(&self) {}

    pub fn insert_partition_opening(&self) {}

    pub fn subdivide_partition(&self) {}
}

/*pub fn generate_random_maze_genome() -> MazeGenome {
    MazeGenome::new()
}

pub fn generate_random_mazes(amount: i32) -> Vec<MazeGenome> {
    let new_maze = generate_random_maze_genome();
    vec![new_maze]
}*/

