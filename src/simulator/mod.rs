use crate::maze::maze_phenotype::{MazeCell, MazePhenotype};
use crate::neatns::agent::Agent;
use crate::network::neural_network::NeuralNetwork;
use crate::simulator::sensor::get_all_sensor_values;

mod sensor;

pub struct RunState {
    current_cell_x: u32,
    current_cell_y: u32,
    current_x_in_cell: f64,
    current_y_in_cell: f64,
    rotation_offset: f64,
    /* 0 - 359 */
}

impl RunState {
    pub fn new() -> RunState {
        RunState {
            current_cell_x: 0,
            current_cell_y: 0,
            current_x_in_cell: 0.5,
            current_y_in_cell: 0.5,
            rotation_offset: 0.0,
        }
    }
}

pub struct SimulatorResult {
    completed: bool,
    distance_from_goal: f64,
}

impl SimulatorResult {
    pub fn new(completed: bool, distance_from_goal: f64) -> SimulatorResult {
        SimulatorResult {
            completed,
            distance_from_goal,
        }
    }
}

pub fn simulate_run(agent: &Agent, maze: &MazePhenotype) -> SimulatorResult {
    let mut steps_left = 1;
    let mut maze_completed = false;
    let mut run_state = RunState::new();

    let sensor_values = get_all_sensor_values(&run_state, maze);
    let mut outputs = vec![0.0, 0.0]; // how far to travel, rotation: [-90, 90]

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