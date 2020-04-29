use core::fmt;
use std::cmp::max;
use std::i32;

use rand::{thread_rng, Rng};

use crate::config;
use crate::maze::maze_phenotype::MazePhenotype;
use crate::maze::maze_validator::MazeValidator;
use crate::maze::{OpeningLocation, Orientation, PathDirection};

#[derive(Debug, Copy, Clone)]
pub struct WallGene {
    pub(crate) wall_position: f64,
    pub(crate) passage_position: f64,
    pub(crate) orientation: Orientation,
    pub(crate) opening_location: OpeningLocation,
}

impl WallGene {
    pub fn new(
        wall_position: f64,
        passage_position: f64,
        orientation: Orientation,
        opening_location: OpeningLocation,
    ) -> WallGene {
        WallGene {
            wall_position,
            passage_position,
            orientation,
            opening_location,
        }
    }

    pub fn set_wall_position(&mut self, value: f64) {
        self.wall_position = value;
    }

    pub fn set_passage_position(&mut self, value: f64) {
        self.passage_position = value;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PathGene {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl PathGene {
    pub fn new(x: u32, y: u32) -> PathGene {
        PathGene { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct MazeGenome {
    pub width: u32,
    pub height: u32,
    first_direction: Orientation,
    pub path_genes: Vec<PathGene>,
    wall_genes: Vec<WallGene>,
    pub(crate) viable: bool,
    pub(crate) id: u32,
    pub successful_agent_id: Option<u32>,
}

impl MazeGenome {
    pub fn new(
        width: u32,
        height: u32,
        first_direction: Orientation,
        path_genes: Vec<PathGene>,
        wall_genes: Vec<WallGene>,
        id: u32,
    ) -> MazeGenome {
        MazeGenome {
            width,
            height,
            first_direction,
            path_genes,
            wall_genes,
            viable: true,
            id,
            successful_agent_id: None,
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let mut distance = 0.0;

        let mut index = 0;

        loop {
            let mut self_value = PathGene::new(0, 0);
            let mut other_value = PathGene::new(0, 0);

            if self.path_genes.len() > index {
                self_value = self.path_genes[index].clone();
            }

            if other.path_genes.len() > index {
                other_value = other.path_genes[index].clone();
            }

            distance += ((other_value.x as f64 - self_value.x as f64).powi(2) as f64
                + (other_value.y as f64 - self_value.y as f64).powi(2) as f64)
                .sqrt();

            index += 1;

            if self.path_genes.len() <= index && other.path_genes.len() <= index {
                break;
            }
        }

        let _wall_gene_scalar_length = max(self.wall_genes.len(), other.wall_genes.len());

        let self_scalar: Vec<f64> = self.wall_gene_scalar_padded();
        let other_scalar: Vec<f64> = other.wall_gene_scalar_padded();

        distance += n_euclidean(self_scalar, other_scalar);

        distance
    }

    fn wall_gene_scalar_padded(&self) -> Vec<f64> {
        let mut scalar: Vec<f64> = vec![];

        for w in self.wall_genes.iter() {
            let value: f64 = 0.5
                * (w.wall_position + w.passage_position)
                * (w.wall_position + w.passage_position + 1.0)
                + w.passage_position;
            scalar.push(value);
        }

        scalar
    }

    pub fn get_amount_of_junctures(&self) -> u32 {
        let mut sum: u32 = 0;

        let mut previous_x = 0;
        let mut previous_y = self.height - 1;

        for (i, path_gene) in self.path_genes.iter().enumerate() {
            if path_gene.x != previous_x && path_gene.y != previous_y {
                sum += 1;
            }

            if i < self.path_genes.len() - 1 {
                if path_gene.x != self.path_genes[i + 1].x && path_gene.y != self.path_genes[i + 1].y {
                    sum += 1;
                }
            } else {
                if path_gene.x != self.width - 1 && path_gene.y != 0 {
                    sum += 1;
                } else if self.first_direction == Orientation::Vertical && path_gene.x == self.width - 1 && path_gene.y != 0 {
                    sum += 1;
                } else if self.first_direction == Orientation::Horizontal && path_gene.x != self.width - 1 && path_gene.y == 0 {
                    sum += 1;
                }


            }

            previous_x = path_gene.x;
            previous_y = path_gene.y;
        }

        if previous_x != self.width - 1 && previous_y != 0 {
            sum += 1;
        }

        sum
    }

    pub fn get_solution_path_cell_length(&self) -> u32 {
        let mut length: u32 = 0;

        let mut current_x: i32 = 0;
        let mut current_y: i32 = self.height as i32 - 1;

        for path_gene in self.path_genes.iter() {
            length += (path_gene.x as i32 - current_x).abs() as u32;
            length += (path_gene.y as i32 - current_y).abs() as u32;

            current_x = path_gene.x as i32;
            current_y = path_gene.y as i32;
        }

        length += ((self.width - 1) as i32 - current_x).abs() as u32;
        length += (0 as i32 - current_y).abs() as u32;

        length
    }

    pub fn to_phenotype(&self) -> MazePhenotype {
        let phenotype = MazePhenotype::new(
            self.width,
            self.height,
            self.first_direction,
            &self.path_genes,
            &self.wall_genes,
        );
        phenotype
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < config::MAZE.mutate_wall {
            self.mutate_wall();
        }

        if rng.gen::<f64>() < config::MAZE.mutate_passage {
            self.mutate_passage();
        }

        if rng.gen::<f64>() < config::MAZE.mutate_waypoint {
            self.mutate_waypoint();
        }

        if rng.gen::<f64>() < config::MAZE.add_wall {
            self.add_wall();
        }

        if rng.gen::<f64>() < config::MAZE.delete_wall {
            self.delete_wall();
        }

        if rng.gen::<f64>() < config::MAZE.add_waypoint {
            for _ in 0..10 {
                let added = self.add_waypoint();

                if added {
                    break;
                }
            }
        }

        if rng.gen::<f64>() < config::MAZE.increase_size {
            self.increase_size();
        }
    }

    pub fn mutate_wall(&mut self) {
        let mut rng = thread_rng();

        let index = (rng.gen::<f32>() * self.wall_genes.len() as f32) as usize;
        self.wall_genes[index].set_wall_position(rng.gen::<f64>());
    }

    pub fn mutate_passage(&mut self) {
        let mut rng = thread_rng();

        let index = (rng.gen::<f32>() * self.wall_genes.len() as f32) as usize;
        self.wall_genes[index].set_passage_position(rng.gen::<f64>());
    }

    pub fn mutate_waypoint(&mut self) {
        let mut rng = thread_rng();

        let index = (rng.gen::<f32>() * self.path_genes.len() as f32) as usize;
        let available_directions: Vec<PathDirection> = vec![
            PathDirection::North,
            PathDirection::East,
            PathDirection::South,
            PathDirection::West,
        ];

        let mut mutation_is_valid = false;
        let mut direction_index = (rng.gen::<f32>() * available_directions.len() as f32) as usize;
        let mut direction = available_directions[direction_index].clone();

        for _ in 0..4 {
            mutation_is_valid = self.validate_path_mutation_direction(index, direction);

            if mutation_is_valid {
                break;
            } else {
                direction_index = (direction_index + 1) % available_directions.len();
                direction = available_directions[direction_index].clone();
            }
        }

        if !mutation_is_valid {
            return;
        }

        if !self.valid_phenotype_after_waypoint_mutation(index, direction) {
            return;
        }

        if direction == PathDirection::North {
            self.path_genes[index].y += 1;
        } else if direction == PathDirection::East {
            self.path_genes[index].x += 1;
        } else if direction == PathDirection::South {
            self.path_genes[index].y -= 1;
        } else if direction == PathDirection::West {
            self.path_genes[index].x -= 1;
        }
    }

    pub fn validate_path_mutation_direction(
        &self,
        gene_index: usize,
        direction: PathDirection,
    ) -> bool {
        let gene = &self.path_genes[gene_index];
        let point_before = if gene_index == 0 {
            PathGene::new(0, self.height - 1)
        } else {
            self.path_genes[gene_index - 1]
        };

        let point_after = if gene_index == self.path_genes.len() - 1 {
            PathGene::new(self.width - 1, 0)
        } else {
            self.path_genes[gene_index + 1]
        };

        if direction == PathDirection::North {
            if gene.y >= self.height - 1 {
                return false;
            }

            if point_after.y == gene.y + 1 || point_before.y == gene.y + 1 {
                return false;
            }
        } else if direction == PathDirection::East {
            if gene.x >= self.width - 1 {
                return false;
            }

            if point_after.x == gene.x + 1 || point_before.x == gene.x + 1 {
                return false;
            }
        } else if direction == PathDirection::South {
            if gene.y <= 0 {
                return false;
            }

            if point_after.y == gene.y - 1 || point_before.y == gene.y - 1 {
                return false;
            }
        } else if direction == PathDirection::West {
            if gene.x <= 0 {
                return false;
            }

            if point_after.x == gene.x - 1 || point_before.x == gene.x - 1 {
                return false;
            }
        }

        true
    }

    pub fn valid_phenotype_after_waypoint_mutation(
        &self,
        index: usize,
        direction: PathDirection,
    ) -> bool {
        let mut clone = self.clone();

        if direction == PathDirection::North {
            clone.path_genes[index].y += 1;
        } else if direction == PathDirection::East {
            clone.path_genes[index].x += 1;
        } else if direction == PathDirection::South {
            clone.path_genes[index].y -= 1;
        } else if direction == PathDirection::West {
            clone.path_genes[index].x -= 1;
        }

        MazeValidator::validate_new_path(
            clone.width,
            clone.height,
            clone.first_direction,
            &clone.path_genes,
        )
    }

    pub fn add_wall(&mut self) {
        let mut rng = rand::thread_rng();

        self.wall_genes.push(WallGene::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            get_random_orientation(rng.gen::<f32>()),
            get_random_opening(rng.gen::<f32>()),
        ));
    }

    pub fn delete_wall(&mut self) {
        if self.wall_genes.len() <= 1 {
            return;
        }
        let mut rng = rand::thread_rng();

        let index = (rng.gen::<f32>() * self.wall_genes.len() as f32) as usize;
        self.wall_genes.remove(index);
    }

    pub fn add_waypoint(&mut self) -> bool {
        let mut rng = rand::thread_rng();

        let path_gene = PathGene::new(
            (rng.gen::<f32>() * self.width as f32) as u32,
            (rng.gen::<f32>() * self.height as f32) as u32,
        );

        if path_gene.x == self.path_genes[self.path_genes.len() - 1].x
            || path_gene.y == self.path_genes[self.path_genes.len() - 1].y
        {
            return false;
        }

        let mut clone = self.clone();

        let validator = MazeValidator::new(
            clone.width,
            clone.height,
            clone.first_direction,
            &clone.path_genes,
        );

        let cell = validator.get_cell_at(path_gene.x, path_gene.y);

        if cell.path_direction == PathDirection::None {
            clone.path_genes.push(path_gene);

            if MazeValidator::validate_new_path(
                clone.width,
                clone.height,
                clone.first_direction,
                &clone.path_genes,
            ) {
                self.path_genes.push(path_gene);
                return true;
            } else {
                return false;
            }
        }
        return false;
    }

    pub fn increase_size(&mut self) {
        self.height += 1;
        self.width += 1;
    }
}

impl fmt::Display for MazeGenome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MazeGenome: {}x{} \n First direction: {:?} \n Path genes: {:?} \n Wall genes {:?}",
            self.width, self.height, self.first_direction, self.path_genes, self.wall_genes
        )
    }
}

pub fn get_random_opening(number: f32) -> OpeningLocation {
    if number < 0.25 {
        OpeningLocation::North
    } else if number >= 0.25 && number < 0.5 {
        OpeningLocation::East
    } else if number >= 0.5 && number < 0.75 {
        OpeningLocation::South
    } else {
        OpeningLocation::West
    }
}

pub fn get_random_orientation(number: f32) -> Orientation {
    if number > 0.5 {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    }
}

pub fn generate_random_maze(width: u32, height: u32, id: u32) -> MazeGenome {
    let mut rng = rand::thread_rng();

    let initial_orientation = get_random_orientation(rng.gen::<f32>());
    let wall_gene = WallGene::new(
        rng.gen::<f64>(),
        rng.gen::<f64>(),
        get_random_orientation(rng.gen::<f32>()),
        get_random_opening(rng.gen::<f32>()),
    );

    if initial_orientation == Orientation::Horizontal {
        let path_gene = PathGene::new(
            1 + (rng.gen::<f32>() * (width - 2) as f32) as u32,
            1 + (rng.gen::<f32>() * (height - 2) as f32) as u32,
        );

        let path_genes = vec![path_gene];
        let wall_genes = vec![wall_gene];

        let mut new_maze = MazeGenome::new(
            width,
            height,
            initial_orientation,
            path_genes,
            wall_genes,
            id,
        );
        new_maze.add_waypoint();
        new_maze.add_waypoint();
        new_maze.add_wall();

        new_maze
    } else {
        let path_gene = PathGene::new(
            1 + (rng.gen::<f32>() * (width - 2) as f32) as u32,
            height - 1 - (rng.gen::<f32>() * (height - 1) as f32) as u32,
        );

        let path_genes = vec![path_gene];
        let wall_genes = vec![wall_gene];

        let mut new_maze = MazeGenome::new(
            width,
            height,
            initial_orientation,
            path_genes,
            wall_genes,
            id,
        );
        new_maze.add_waypoint();
        new_maze.add_waypoint();
        new_maze.add_wall();

        new_maze
    }
}

pub fn n_euclidean(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let mut distance: f64 = 0.0;

    for (a, b) in a.iter().zip(b.iter()) {
        distance += (b - a).powi(2);
    }

    distance.sqrt()
}
