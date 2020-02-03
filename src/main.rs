use crate::maze::{MazeGenome, PathGene, WallGene, Orientation};
use envconfig::Envconfig;

mod maze;
mod navigator;
mod testing;
mod evolution;
mod simulator;

#[derive(Envconfig)]
pub struct MazeMutationOptions {
    mutate_structure_probability: f32,
    add_wall_probability: f32,
    delete_wall_probability: f32,
    add_waypoint_probability: f32,
    delete_waypoint_probability: f32,
}

pub struct NavigatorMutationOptions {
    mutate_weight_probability: f32,
    add_connection_probability: f32,
    add_neuron_probability: f32,
    delete_neuron_probability: f32,
}

pub struct MCCOptions {
    generations: i32,
    maze_population_capacity: i32,
    maze_seed_amount: i32,
    navigator_population_capacity: usize,
    navigator_seed_amount: i32,
    maze_mutation_options: MazeMutationOptions,
    navigator_mutation_options: NavigatorMutationOptions,
}

fn run_mcc_plain(options: MCCOptions) {
    /*let mazes = maze::generate_random_mazes(options.maze_population_capacity);
    let viable_navigators = evolution::evolve_seed_navigators(&mazes, options.navigator_seed_amount);

    for x in 0..options.generations {
        println!("Generation {}", x);

        let parents = evolution::dequeue(&viable_navigators, 10);
        let children = evolution::reproduce_navigators(parents);
        evolution::enqueue(&viable_navigators, parents);

        //

        let survivors = simulator::evaluate_navigators(&children, &mazes);
        evolution::enqueue(&viable_navigators, &survivors);

        if viable_navigators.len() > options.navigator_population_capacity {
            let amount_to_remove = viable_navigators.len() - options.navigator_population_capacity;
            evolution::remove_oldest(&viable_navigators, amount_to_remove)
        }
    }*/
}


//fn run_mcc_speciated() {}


fn main() {
    let p1 = PathGene::new(2, 3, Orientation::Vertical);
    let p2 = PathGene::new(5, 6, Orientation::Vertical);

    let w1 = WallGene::new(0.278, 0.855);
    let w2 = WallGene::new(400.0, 0.808);

    let mazey_boi = MazeGenome::new(10, 10, vec![p1, p2], vec![w1, w2]);

    let pheno = mazey_boi.to_phenotype();
    pheno.visualize();
}
