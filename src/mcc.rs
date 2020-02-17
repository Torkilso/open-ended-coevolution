/*fn run_mcc_plain() {
    let viable_mazes = maze::generate_random_mazes(options.maze_population_capacity);
    let viable_navigators = evolution::evolve_seed_navigators(&viable_mazes, options.navigator_seed_amount);

    for x in 0..options.generations {
        println!("Generation {}", x);

        let maze_parents = evolution::dequeue(&viable_mazes, 10);
        let maze_children = evolution::reproduce_mazes(navigator_parents);

        let navigator_parents = evolution::dequeue(&viable_navigators, 10);
        let navigator_children = evolution::reproduce_navigators(navigator_parents);
        evolution::enqueue(&viable_navigators, navigator_parents);

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
        println!("Generation {}", x);

        let all_species = find_species(viable_navigators);

        let maze_parents = evolution::dequeue(&viable_mazes, 10);
        let maze_children = evolution::reproduce_mazes(navigator_parents);

        for species in all_species {
            // choose random maze to use

            let navigator_parents = evolution::dequeue(&species, 10);
            let navigator_children = evolution::reproduce_navigators(species);
                    evolution::enqueue(&viable_navigators, navigator_parents);
            let (navigator_survivors, maze_survivors) = simulator::evaluate_navigators(&navigator_children, &maze_children);

            evolution::enqueue(&viable_navigators, &navigator_survivors);

        }

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