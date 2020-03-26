use crate::neatns;
use crate::config;

pub mod mcc;
mod agent_queue;

pub fn run() {

    let seeds = neatns::generate_seeds();

    //let maze queue = seeds.mazes

    let mut generation = 0;

    while generation < config::MCC.generations {

        // speciate
        // selection from queue
        // evolve
        // evaluate
        // add viable children to queue





        /*speciation::speciate_mazes(&viable_mazes);
        speciation::speciate_navigators(&viable_navigators);

        let maze_children = genetics::reproduce_mazes(&viable_mazes);
        let navigator_children = genetics::reproduce_navigators(&viable_navigators);

        let (navigator_survivors, maze_survivors) = simulator::evaluate_navigators(&navigator_children, &maze_children);

        population::enqueue(&viable_mazes, &maze_survivors);
        population::enqueue(&viable_navigators, &navigator_survivors);

        if viable_navigators.len() > MCC.navigator_population_capacity {
            let amount_to_remove = viable_navigators.len() - MCC.navigator_population_capacity;
            population::remove_oldest(&viable_navigators, amount_to_remove)
        }

        if viable_mazes.len() > MCC.maze_population_capacity {
            let amount_to_remove = viable_mazes.len() - options.maze_population_capacity;
            population::remove_oldest(&viable_mazes, amount_to_remove)
        }*/


        generation += 1;
    }
}