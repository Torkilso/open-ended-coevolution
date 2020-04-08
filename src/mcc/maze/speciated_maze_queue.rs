/*use crate::maze::maze_genotype::MazeGenome;

pub struct MazeSpecies {
    id: u32,
    agents: Vec<MazeGenome>,
    pub(crate) complexity_growth: f64,
}

pub struct SpeciatedMazeQueue {
    species: Vec<MazeSpecies>,
}

impl SpeciatedMazeQueue {
    pub fn new(mazes: Vec<MazeGenome>) -> SpeciatedMazeQueue {
        let queue = SpeciatedMazeQueue { species: vec![] };

        for maze in mazes.iter() {
            queue.add_agent(maze.clone())
        }

        queue
    }

    pub fn speciate(&self) {}

    pub fn add_agent(&self, maze: MazeGenome) {
        // check if agent fits into any species
        // add to species if yes, create new species if no
    }
}
*/
