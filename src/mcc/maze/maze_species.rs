use crate::maze::maze_genotype::MazeGenome;

pub struct MazeSpecies {
    id: u32,
    agents: Vec<MazeGenome>,
    pub(crate) complexity_growth: f64,
}
