use crate::maze::maze_phenotype::{MazeCell, MazePhenotype};
use crate::neatns::agent::Agent;
use crate::network::neural_network::NeuralNetwork;
use crate::simulator::radar::get_radar_values;
use crate::simulator::run_state::RunState;
use crate::config;
use std::fmt;

mod sensor;
pub mod radar;
mod run_state;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulatorResult {
    agent_reached_end: bool,
    agent_path: Vec<Point>,
}

impl SimulatorResult {
    pub fn new() -> SimulatorResult {
        SimulatorResult {
            agent_reached_end: false,
            agent_path: vec![],
        }
    }

    pub fn agent_reached_end(&self) -> bool {
        self.agent_reached_end
    }

    pub fn set_agent_reached_end(&mut self, value: bool) {
        self.agent_reached_end = value;
    }

    pub fn add_point (&mut self, point: Point) {
        self.agent_path.push(point);
    }

    pub fn final_position(&self) -> Option<&Point>{
        self.agent_path.last()
    }
}

impl fmt::Display for SimulatorResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Simulator result: \nCompleted: {} \nPath: {:?}", self.agent_reached_end, self.agent_path)
    }
}

pub fn simulate_run(agent: &Agent, maze: &MazePhenotype) -> SimulatorResult {
    let mut steps_left = 100;
    let mut run_state = RunState::new(maze.height);

    let mut agent_phenotype = agent.to_phenotype();

    let mut result = SimulatorResult::new();

    while steps_left > 0 {
        let sensor_values = run_state.get_all_sensor_values(maze);
        let radar_values = get_radar_values(&run_state, maze).to_f64_vector();
        let all_inputs = [&sensor_values[..], &radar_values[..]].concat();

        let output = agent_phenotype.activate(&all_inputs);

        run_state.update_velocities(output[0], output[1]);

        //println!("velocities: {} {}", output, run_state.current_velocity, run_state.current_angular_velocity);
        //println!("outputs: {:?}", output);

        let new_position = run_state.update_position(maze);

        result.add_point(new_position);

        //println!("position: {} {}", run_state.global_x, run_state.global_y);

        if run_state.maze_completed(maze.width) {
            result.set_agent_reached_end(true);
            return result;
        }

        steps_left -= 1;
    }

    result
}

