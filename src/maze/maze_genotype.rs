extern crate queues;

use rand::Rng;

use crate::maze::maze_phenotype::MazePhenotype;
use crate::maze::{Orientation, OpeningLocation};
use std::fmt::Display;
use core::fmt;

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
    width: u32,
    height: u32,
    first_direction: Orientation,
    path_genes: Vec<PathGene>,
    wall_genes: Vec<WallGene>,
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
}

impl fmt::Display for MazeGenome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MazeGenome: {}x{} \n First direction: {:?} \n Path genes: {:?} \n Wall genes {:?}", self.width, self.height, self.first_direction, self.path_genes, self.wall_genes)
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
        let path_gene = PathGene::new(1 + (rng.gen::<f32>() * (width - 2) as f32) as u32, (rng.gen::<f32>() * height as f32) as u32);

        let path_genes = vec![path_gene];
        let wall_genes = vec![wall_gene];

        MazeGenome::new(width, height, initial_orientation, path_genes, wall_genes)
    } else {
        let path_gene = PathGene::new((rng.gen::<f32>() * width as f32) as u32, height - 1 - (rng.gen::<f32>() * (height - 1) as f32) as u32);

        let path_genes = vec![path_gene];
        let wall_genes = vec![wall_gene];

        MazeGenome::new(width, height, initial_orientation, path_genes, wall_genes)
    }
}