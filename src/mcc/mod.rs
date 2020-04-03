use crate::config;
use crate::mcc::agent::agent_queue::AgentQueue;
use crate::mcc::maze::maze_queue::MazeQueue;
use crate::neatns;
use crate::simulator::simulate_many;

pub(crate) mod agent;
mod maze;

pub fn run_without_speciation() {
    let seeds = neatns::generate_seeds();

    let mut agents = AgentQueue::new(seeds.agents);
    let mut mazes = MazeQueue::new(seeds.mazes);

    for generation in 0..config::MCC.generations {
        let mut agent_children = agents.get_children();
        let mut maze_children = mazes.get_children();

        simulate_many(&mut agent_children, &mut maze_children);

        for child in agent_children.iter() {
            if child.viable {
                agents.push(child.clone())
            }
        }

        for child in maze_children.iter() {
            if child.viable {
                mazes.push(child.clone())
            }
        }

        println!("Generation: {}\tAgents: {}\tMazes: {}", generation, agents.len(), mazes.len());
    }
}
