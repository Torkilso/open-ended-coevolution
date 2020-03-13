use crate::maze::maze_phenotype::{MazeCell, MazePhenotype};
use crate::neatns::agent::Agent;
use crate::network::neural_network::NeuralNetwork;
use crate::simulator::radar::get_radar_values;
use crate::simulator::run_state::RunState;
use crate::config;

mod sensor;
pub mod radar;
mod run_state;

pub struct SimulatorResult {
    agent_reached_end: bool,
    distance_from_goal: f64,
}

impl SimulatorResult {
    pub fn new(agent_reached_end: bool, distance_from_goal: f64) -> SimulatorResult {
        SimulatorResult {
            agent_reached_end,
            distance_from_goal,
        }
    }

    pub fn agent_reached_end(&self) -> bool {
        self.agent_reached_end
    }
}

pub fn simulate_run(agent: &Agent, maze: &MazePhenotype) -> SimulatorResult {
    let mut steps_left = 100;
    let mut run_state = RunState::new();

    let mut agent_phenotype = agent.to_phenotype();

    while steps_left > 0 {
        let sensor_values = run_state.get_all_sensor_values(maze);
        let radar_values = get_radar_values(&run_state, maze).to_f64_vector();
        let all_inputs = [&sensor_values[..], &radar_values[..]].concat();

        let output = agent_phenotype.activate(&all_inputs);

        run_state.update_velocities(output[0], output[1]);
        run_state.update_position(maze);

        //println!("{} {}", run_state.current_x_in_cell, run_state.current_y_in_cell);

        if run_state.maze_completed(maze.width, maze.height) {
            return SimulatorResult::new(true, run_state.distance_from_goal());
        }

        steps_left -= 1;
    }

    SimulatorResult::new(false, run_state.distance_from_goal())
}

