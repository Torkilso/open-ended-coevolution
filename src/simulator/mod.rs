use crate::config;
use crate::maze::maze_phenotype::{MazeCell, MazePhenotype};
use crate::neatns::agent::Agent;
use crate::network::neural_network::NeuralNetwork;
use crate::simulator::radar::get_radar_values;
use crate::simulator::run_state::RunState;
use std::fmt;

pub mod radar;
mod run_state;
mod sensor;

#[derive(Debug, Clone)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct SimulatorResult {
    pub(crate) agent_reached_end: bool,
    pub(crate) agent_path: Vec<Point>,
    pub(crate) final_position: Option<Point>,
}

impl SimulatorResult {
    pub fn new() -> SimulatorResult {
        SimulatorResult {
            agent_reached_end: false,
            agent_path: vec![],
            final_position: Option::None,
        }
    }

    pub fn agent_reached_end(&self) -> bool {
        self.agent_reached_end
    }

    pub fn set_agent_reached_end(&mut self, value: bool) {
        self.agent_reached_end = value;
    }

    pub fn add_point(&mut self, point: Point) {
        self.agent_path.push(point);
    }

    pub fn final_position(&self) -> Option<&Point> {
        self.agent_path.last()
    }
}

impl fmt::Display for SimulatorResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Simulator result: \nCompleted: {} \nPath: {:?}",
            self.agent_reached_end, self.agent_path
        )
    }
}

pub fn simulate_run(agent: &Agent, maze: &MazePhenotype, trace_path: bool) -> SimulatorResult {
    let mut steps_left = 1000;
    let mut run_state = RunState::new(maze.height);

    let mut agent_phenotype = agent.to_phenotype();

    let mut result = SimulatorResult::new();

    while steps_left > 0 {
        let sensor_values = run_state.get_all_sensor_values(maze);
        let radar_values = get_radar_values(&run_state, maze).to_f64_vector();
        let all_inputs = [&sensor_values[..], &radar_values[..]].concat();

        //println!("inputs: {:?}", all_inputs);

        let output = agent_phenotype.activate(&all_inputs);

        run_state.update_velocities(output[0], output[1]);

        /*println!(
            "velocities adjustment: {:?} | new: {} {}",
            output, run_state.current_velocity, run_state.current_angular_velocity
        );*/

        let new_position = run_state.update_position(maze);

        if trace_path {
            result.add_point(new_position.clone());
        }

        //println!("position: {} {}", run_state.global_x, run_state.global_y);

        if run_state.maze_completed(maze.width) {
            result.final_position = Option::Some(new_position.clone());
            result.set_agent_reached_end(true);
            return result;
        }

        steps_left -= 1;

        if steps_left == 0 {
            result.final_position = Option::Some(new_position.clone());
        }
    }

    result
}
