use crate::maze::maze_phenotype::{MazeCell, MazePhenotype};
use crate::neatns::agent::Agent;
use crate::network::neural_network::NeuralNetwork;
use crate::simulator::sensor::get_all_sensor_values;
use crate::simulator::radar::get_radar_values;
use crate::config;

mod sensor;
pub mod radar;

pub struct RunState {
    current_cell_x: u32,
    current_cell_y: u32,
    current_x_in_cell: f64,
    current_y_in_cell: f64,
    direction: f64,
    /* 0 - 359 */
}

impl RunState {
    pub fn new() -> RunState {
        RunState {
            current_cell_x: 0,
            current_cell_y: 0,
            current_x_in_cell: 0.5,
            current_y_in_cell: 0.5,
            direction: config::AGENT.start_offset,
        }
    }

    pub fn global_x(&self) -> f64 {
        self.current_cell_x as f64 + self.current_x_in_cell
    }

    pub fn global_y(&self, maze_height: u32) -> f64 {
        (maze_height - self.current_cell_y) as f64 - self.current_y_in_cell
    }
}

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
    let mut maze_completed = false;
    let mut run_state = RunState::new();

    let sensor_values = get_all_sensor_values(&run_state, maze);
    let radar_values = get_radar_values(&run_state, maze).to_f64_vector();

    let all_inputs = [&sensor_values[..], &radar_values[..]].concat();

    let mut agent_phenotype = agent.to_phenotype();

    println!("inputs: {:#?}", all_inputs);

    let output = agent_phenotype.activate(&all_inputs);

    println!("outputs: {:#?}", output);


    /*steps_left -= 1;

    while steps_left > 0 && !maze_completed {
        let sensor_values = get_all_sensor_values(&run_state, maze);

        let mut outputs = vec![0.0, 0.0]; // travel distance and rotation
        agent.activate(&sensor_values, &mut outputs);

        println!("outputs: {:?}", outputs);

        /*let movement = navigator.get_movement(sensor_values);
        let new_position = get_new_positions(x, y, movement);

        x = new_position.x;
        y = new_position.y;*/


        steps_left -= 1;
    }*/

    SimulatorResult::new(maze_completed, 1.0)
}

