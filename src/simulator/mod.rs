use crate::config;
use crate::maze::maze_genotype::MazeGenome;
use crate::maze::maze_phenotype::MazePhenotype;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::neatns::agent::Agent;
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

    //pub fn final_position(&self) -> Option<&Point> {
    //    self.agent_path.last()
    //}
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

pub fn simulate_single_neatns(
    agent: &Agent,
    maze: &MazePhenotype,
    length: u32,
    trace_path: bool,
) -> SimulatorResult {
    let mut steps_left = length * config::MAZE.cell_dimension as u32;
    let mut run_state = RunState::new(maze.height);

    let mut agent_phenotype = agent.to_phenotype();

    let mut result = SimulatorResult::new();

    while steps_left > 0 {
        let sensor_values = run_state.get_all_sensor_values(maze);
        let radar_values = get_radar_values(&run_state, maze).to_f64_vector();
        let all_inputs = [&sensor_values[..], &radar_values[..]].concat();

        let output = agent_phenotype.activate(&all_inputs);
        run_state.update_velocities(output[0], output[1]);
        let new_position = run_state.update_position(maze);

        if trace_path {
            result.add_point(new_position.clone());
        }

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

pub fn simulate_single_mcc(
    agent: &MCCAgent,
    maze: &MazePhenotype,
    length: u32,
    trace_path: bool,
) -> SimulatorResult {
    let mut steps_left = length * config::MAZE.cell_dimension as u32;
    let mut run_state = RunState::new(maze.height);

    let mut agent_phenotype = agent.to_phenotype();

    let mut result = SimulatorResult::new();

    while steps_left > 0 {
        let sensor_values = run_state.get_all_sensor_values(maze);
        let radar_values = get_radar_values(&run_state, maze).to_f64_vector();
        let all_inputs = [&sensor_values[..], &radar_values[..]].concat();

        let output = agent_phenotype.activate(&all_inputs);
        run_state.update_velocities(output[0], output[1]);

        let new_position = run_state.update_position(maze);

        if trace_path {
            result.add_point(new_position.clone());
        }

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

// Simulates each agent in all mazes, marks viable agents and mazes that fulfill MC
pub fn simulate_many(agents: &mut Vec<MCCAgent>, mazes: &mut Vec<MazeGenome>) {
    for maze in mazes.iter_mut() {
        let maze_phenotype = maze.to_phenotype();
        for agent in agents.iter_mut() {
            let simulator_result = simulate_single_mcc(
                agent,
                &maze_phenotype,
                maze.get_solution_path_cell_length(),
                false,
            );

            if simulator_result.agent_reached_end {
                agent.viable = true;
                maze.viable = true;
                maze.successful_agent_id = Some(agent.id)
            }
        }
    }
}
