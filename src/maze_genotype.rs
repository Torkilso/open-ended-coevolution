extern crate queues;

use math::round;
use queues::*;

use crate::general::Orientation;
use crate::maze_phenotype::MazePhenotype;
use crate::MazeMutationOptions;

#[derive(Debug, Clone)]
pub struct WallGene {
    pub(crate) wall_position: f32,
    pub(crate) passage_position: f32,
    pub(crate) orientation: Orientation,
}

impl WallGene {
    pub fn new(wall_position: f32, passage_position: f32, orientation: Orientation) -> WallGene {
        WallGene {
            wall_position,
            passage_position,
            orientation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PathGene {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) orientation: Orientation,
}

impl PathGene {
    pub fn new(x: u32, y: u32, orientation: Orientation) -> PathGene {
        PathGene { x, y, orientation }
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
    pub fn new(
        width: u32,
        height: u32,
        path_genes: Vec<PathGene>,
        wall_genes: Vec<WallGene>,
    ) -> MazeGenome {
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

        let mut phenotype = MazePhenotype::new(self.width, self.height, self.get_path_genes(), self.get_wall_genes());
        phenotype
    }
}
