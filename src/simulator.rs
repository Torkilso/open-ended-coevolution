use crate::agent::agent::Agent;
use crate::maze::maze_phenotype::{MazeCell, MazePhenotype};

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

pub fn find_sensor_value_north_east(
    angle: f64,
    current_x_in_cell: f64,
    current_y_in_cell: f64,
    current_cell_x: u32,
    current_cell_y: u32,
    maze: &MazePhenotype,
) -> f64 {
    let mut missing_side: f64 = (1.0 - current_x_in_cell) * angle.to_radians().tan();

    if missing_side + current_y_in_cell < 1.0 {
        let hypotenuse = ((1.0 - current_x_in_cell).powi(2) + missing_side.powi(2)).sqrt();

        if maze.get_cell_at(current_cell_x, current_cell_y).east_wall {
            hypotenuse
        } else {
            hypotenuse
                + find_sensor_value_north_east(
                    angle,
                    0.0,
                    missing_side + current_y_in_cell,
                    current_cell_x + 1,
                    current_cell_y,
                    maze,
                )
        }
    } else {
        missing_side = (1.0 - current_y_in_cell) * (90.0 - angle).to_radians().tan();
        let hypotenuse = ((1.0 - current_y_in_cell).powi(2) + missing_side.powi(2)).sqrt();

        if maze.get_cell_at(current_cell_x, current_cell_y).north_wall {
            hypotenuse
        } else {
            hypotenuse
                + find_sensor_value_north_east(
                    angle,
                    missing_side + current_x_in_cell,
                    0.0,
                    current_cell_x,
                    current_cell_y - 1,
                    maze,
                )
        }
    }
}

pub fn find_sensor_value_north_west(
    angle: f64,
    current_x_in_cell: f64,
    current_y_in_cell: f64,
    current_cell_x: u32,
    current_cell_y: u32,
    maze: &MazePhenotype,
) -> f64 {
    let calculation_angle = angle - 90.0;
    let mut missing_side: f64 = (1.0 - current_y_in_cell) * calculation_angle.to_radians().tan();

    if current_x_in_cell - missing_side > 0.0 {
        let hypotenuse = ((1.0 - current_y_in_cell).powi(2) + missing_side.powi(2)).sqrt();

        if maze.get_cell_at(current_cell_x, current_cell_y).north_wall {
            hypotenuse
        } else {
            hypotenuse
                + find_sensor_value_north_west(
                    angle,
                    current_x_in_cell - missing_side,
                    0.0,
                    current_cell_x,
                    current_cell_y - 1,
                    maze,
                )
        }
    } else {
        missing_side = current_x_in_cell * (90.0 - calculation_angle).to_radians().tan();
        let hypotenuse = (current_x_in_cell.powi(2) + missing_side.powi(2)).sqrt();

        if maze.get_cell_at(current_cell_x, current_cell_y).west_wall {
            hypotenuse
        } else {
            hypotenuse
                + find_sensor_value_north_west(
                    angle,
                    1.0,
                    missing_side + current_y_in_cell,
                    current_cell_x - 1,
                    current_cell_y,
                    maze,
                )
        }
    }
}

pub fn find_sensor_value_south_west(
    angle: f64,
    current_x_in_cell: f64,
    current_y_in_cell: f64,
    current_cell_x: u32,
    current_cell_y: u32,
    maze: &MazePhenotype,
) -> f64 {
    let calculation_angle = angle - 180.0;
    let mut missing_side: f64 = current_x_in_cell * calculation_angle.to_radians().tan();

    if current_y_in_cell - missing_side > 0.0 {
        let hypotenuse = (current_x_in_cell.powi(2) + missing_side.powi(2)).sqrt();

        if maze.get_cell_at(current_cell_x, current_cell_y).west_wall {
            hypotenuse
        } else {
            hypotenuse
                + find_sensor_value_south_west(
                    angle,
                    1.0,
                    current_y_in_cell - missing_side,
                    current_cell_x - 1,
                    current_cell_y,
                    maze,
                )
        }
    } else {
        missing_side = current_y_in_cell * (90.0 - calculation_angle).to_radians().tan();
        let hypotenuse = (current_y_in_cell.powi(2) + missing_side.powi(2)).sqrt();

        if maze.get_cell_at(current_cell_x, current_cell_y).south_wall {
            hypotenuse
        } else {
            hypotenuse
                + find_sensor_value_south_west(
                    angle,
                    current_x_in_cell - missing_side,
                    1.0,
                    current_cell_x,
                    current_cell_y + 1,
                    maze,
                )
        }
    }
}

pub fn find_sensor_value_south_east(
    angle: f64,
    current_x_in_cell: f64,
    current_y_in_cell: f64,
    current_cell_x: u32,
    current_cell_y: u32,
    maze: &MazePhenotype,
) -> f64 {
    let calculation_angle = angle - 270.0;
    let mut missing_side: f64 = current_y_in_cell * calculation_angle.to_radians().tan();

    if current_x_in_cell + missing_side < 1.0 {
        let hypotenuse = (current_y_in_cell.powi(2) + missing_side.powi(2)).sqrt();

        if maze.get_cell_at(current_cell_x, current_cell_y).south_wall {
            hypotenuse
        } else {
            hypotenuse
                + find_sensor_value_south_east(
                    angle,
                    current_x_in_cell + missing_side,
                    1.0,
                    current_cell_x,
                    current_cell_y + 1,
                    maze,
                )
        }
    } else {
        missing_side = (1.0 - current_x_in_cell) * (90.0 - calculation_angle).to_radians().tan();
        let hypotenuse = ((1.0 - current_x_in_cell).powi(2) + missing_side.powi(2)).sqrt();

        if maze.get_cell_at(current_cell_x, current_cell_y).east_wall {
            hypotenuse
        } else {
            hypotenuse
                + find_sensor_value_south_east(
                    angle,
                    0.0,
                    current_y_in_cell - missing_side,
                    current_cell_x + 1,
                    current_cell_y,
                    maze,
                )
        }
    }
}

pub fn get_sensor_value(angle: f64, run_state: &RunState, maze: &MazePhenotype) -> f64 {
    if angle >= 0.0 && angle < 90.0 {
        find_sensor_value_north_east(
            angle,
            run_state.current_x_in_cell,
            run_state.current_y_in_cell,
            run_state.current_cell_x,
            run_state.current_cell_y,
            maze,
        )
    } else if angle >= 90.0 && angle < 180.0 {
        find_sensor_value_north_west(
            angle,
            run_state.current_x_in_cell,
            run_state.current_y_in_cell,
            run_state.current_cell_x,
            run_state.current_cell_y,
            maze,
        )
    } else if angle >= 180.0 && angle < 270.0 {
        find_sensor_value_south_west(
            angle,
            run_state.current_x_in_cell,
            run_state.current_y_in_cell,
            run_state.current_cell_x,
            run_state.current_cell_y,
            maze,
        )
    } else if angle >= 270.0 && angle < 360.0 {
        find_sensor_value_south_east(
            angle,
            run_state.current_x_in_cell,
            run_state.current_y_in_cell,
            run_state.current_cell_x,
            run_state.current_cell_y,
            maze,
        )
    } else {
        1.0
    }
}

pub fn get_all_sensor_values(run_state: &RunState, maze: &MazePhenotype) -> Vec<f64> {
    let sensor_base_angles: Vec<f64> = vec![0.0, 60.0, 120.0, 180.0, 240.0, 300.0];

    let mut sensor_values: Vec<f64> = Vec::new();

    for angle in sensor_base_angles {
        let value = get_sensor_value((angle + run_state.rotation_offset) % 360.0, run_state, maze);
        sensor_values.push(value);
    }
    sensor_values
}

pub fn simulate_navigator_in_maze(agent: &Agent, maze: &MazePhenotype) -> SimulatorResult {
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
