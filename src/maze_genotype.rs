extern crate queues;

use crate::maze_phenotype::MazePhenotype;
use crate::general::{Orientation, OpeningLocation};

#[derive(Debug, Clone)]
pub struct WallGene {
    pub(crate) wall_position: f32,
    pub(crate) passage_position: f32,
    pub(crate) orientation: Orientation,
    pub(crate) opening_location: OpeningLocation,
}

impl WallGene {
    pub fn new(wall_position: f32, passage_position: f32, orientation: Orientation, opening_location: OpeningLocation) -> WallGene {
        WallGene {
            wall_position,
            passage_position,
            orientation,
            opening_location,
        }
    }
}

#[derive(Debug, Clone)]
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

    /*pub fn mutate(&self) {
        //
    }*/

    pub fn to_phenotype(&self) -> MazePhenotype {
        let phenotype = MazePhenotype::new(self.width, self.height, self.first_direction, self.get_path_genes(), self.get_wall_genes());
        phenotype
    }
}
