use core::fmt;
use std::fmt::Display;

use rand::{Rng, thread_rng};
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

use crate::config;
use crate::maze::{OpeningLocation, Orientation, PathDirection};
use crate::maze::maze_phenotype::MazePhenotype;
use std::borrow::{BorrowMut, Borrow};

#[derive(Debug, Copy, Clone)]
pub struct WallGene {
    pub(crate) wall_position: f32,
    pub(crate) passage_position: f32,
    pub(crate) orientation: Orientation,
    pub(crate) opening_location: OpeningLocation,
}

impl WallGene {
    pub fn new(
        wall_position: f32,
        passage_position: f32,
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

    pub fn set_wall_position(&mut self, value: f32) {
        self.wall_position = value;
    }

    pub fn set_passage_position(&mut self, value: f32) {
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
    path_genes: Vec<PathGene>,
    wall_genes: Vec<WallGene>,
    pub(crate) viable: bool,
}

impl MazeGenome {
    pub fn new(
        width: u32,
        height: u32,
        first_direction: Orientation,
        path_genes: Vec<PathGene>,
        wall_genes: Vec<WallGene>,
    ) -> MazeGenome {
        MazeGenome {
            width,
            height,
            first_direction,
            path_genes,
            wall_genes,
            viable: true,
        }
    }

    pub fn get_path_genes(&self) -> &Vec<PathGene> {
        &self.path_genes
    }

    pub fn get_wall_genes(&self) -> &Vec<WallGene> {
        &self.wall_genes
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
            self.add_waypoint();
        }

        if rng.gen::<f64>() < config::MAZE.increase_size {
            self.increase_size();
        }
    }

    pub fn mutate_wall(&mut self) {
        let mut rng = thread_rng();

        let index = (rng.gen::<f32>() * self.wall_genes.len() as f32) as usize;
        self.wall_genes[index].set_wall_position(rng.gen::<f32>());
    }

    pub fn mutate_passage(&mut self) {
        let mut rng = thread_rng();

        let index = (rng.gen::<f32>() * self.wall_genes.len() as f32) as usize;
        self.wall_genes[index].set_passage_position(rng.gen::<f32>());
    }

    pub fn mutate_waypoint(&mut self) {
        let index = (rng.gen::<f32>() * self.path_genes.len() as f32) as usize;
        let point_before = if index == 0 {
            PathGene::new(0, self.height - 1);
        } else {
            self.path_genes[index - 1];
        };

        let point_after = if index == self.path_genes.len() - 1 {
            PathGene::new(self.width - 1, 0);
        } else {
            self.path_genes[index + 1];
        };

        let available_directions: Vec<PathDirection> = vec!();

        if self.first_direction == Orientation::Vertical {

        } else if self.first_direction == Orientation::Horizontal {

        }

        if available_directions.len() == 0 {
            return;
        }

        let direction = available_directions[(rng.gen::<f32>() * self.available_directions.len() as f32) as usize].borrow();

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

    pub fn add_wall(&mut self) {
        let mut rng = rand::thread_rng();

        self.wall_genes.push(WallGene::new(
            rng.gen::<f32>(),
            rng.gen::<f32>(),
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

    pub fn add_waypoint(&mut self) {
        let x = 1 as u32;
        let y = 1 as u32;
        self.path_genes.push(PathGene::new(
            x,
            y,
        ));
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

fn get_random_opening(number: f32) -> OpeningLocation {
    if number < 0.25 {
        OpeningLocation::North
    } else if number >= 0.25 && number < 0.5 {
        OpeningLocation::East
    } else if number >= 0.25 && number < 0.5 {
        OpeningLocation::South
    } else {
        OpeningLocation::West
    }
}

fn get_random_orientation(number: f32) -> Orientation {
    if number > 0.5 {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    }
}

pub fn generate_random_maze(width: u32, height: u32) -> MazeGenome {
    let mut rng = rand::thread_rng();

    let mut initial_orientation = get_random_orientation(rng.gen::<f32>());
    let wall_gene = WallGene::new(
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        get_random_orientation(rng.gen::<f32>()),
        get_random_opening(rng.gen::<f32>()),
    );

    if initial_orientation == Orientation::Horizontal {
        let path_gene = PathGene::new(
            1 + (rng.gen::<f32>() * (width - 2) as f32) as u32,
            (rng.gen::<f32>() * height as f32) as u32,
        );

        let path_genes = vec![path_gene];
        let wall_genes = vec![wall_gene];

        MazeGenome::new(width, height, initial_orientation, path_genes, wall_genes)
    } else {
        let path_gene = PathGene::new(
            (rng.gen::<f32>() * width as f32) as u32,
            height - 1 - (rng.gen::<f32>() * (height - 1) as f32) as u32,
        );

        let path_genes = vec![path_gene];
        let wall_genes = vec![wall_gene];

        MazeGenome::new(width, height, initial_orientation, path_genes, wall_genes)
    }
}
