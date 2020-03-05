/*fn run_mcc_plain() {
    let viable_mazes = maze::generate_random_mazes(options.maze_population_capacity);
    let viable_navigators = evolution::evolve_seed_navigators(&viable_mazes, options.navigator_seed_amount);

    for x in 0..options.generations {
        let maze_children = genetics::reproduce_mazes(&viable_mazes);
        let navigator_children = genetics::reproduce_navigators(&viable_navigators);

        let (navigator_survivors, maze_survivors) = simulator::evaluate_navigators(&navigator_children, &maze_children);
        evolution::enqueue(&viable_navigators, &navigator_survivors);
        evolution::enqueue(&viable_mazes, &maze_survivors);

        if viable_navigators.len() > MCC.navigator_population_capacity {
            let amount_to_remove = viable_navigators.len() - MCC.navigator_population_capacity;
            evolution::remove_oldest(&viable_navigators, amount_to_remove)
        }

        if viable_mazes.len() > MCC.maze_population_capacity {
            let amount_to_remove = viable_mazes.len() - options.maze_population_capacity;
            evolution::remove_oldest(&viable_mazes, amount_to_remove)
        }
    }
}*/

/*fn run_mcc_speciated() {
    let viable_mazes = maze::generate_random_mazes(options.maze_population_capacity);
    let viable_navigators = evolution::evolve_seed_navigators(&viable_mazes, options.navigator_seed_amount);

    for x in 0..MCC.generations {
        speciation::speciate_mazes(&viable_mazes);
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
        }
    }
}*/
