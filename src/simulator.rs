use crate::navigator::Navigator;
use crate::maze_phenotype::{MazePhenotype, MazeCell};

pub static CELL_TO_UNITS: u32 = 20;

pub struct RunState {
    current_cell_x: u32,
    current_cell_y: u32,
    current_x_in_cell: f32,
    current_y_in_cell: f32,
    angle: f32,
}

// recursive
pub fn find_sensor_value(angle_offset: f32, run_state: &RunState, maze: &MazePhenotype) -> f32 {
    let mut missing_side = (1.0 - run_state.current_x_in_cell) * angle.to_radians().tan();

    1.0

    /*if missing_side + run_state.current_y_in_cell < 1.0 {
        if maze_cell.east_wall {
            ((1.0 - x_in_cell).powi(2) + missing_side.powi(2)).sqrt()
        } else {
            1.0
        }
    } else {
        missing_side = (1.0 - y_in_cell) * (90.0 - angle).to_radians().tan();

        if maze_cell.north_wall {
            (1.0 - y_in_cell).powi(2) + missing_side.powi(2).sqrt()
        } else {
            1.0
        }
    }*/
}

pub fn get_sensor_value(x_in_cell: f32, y_in_cell: f32, angle: f32, maze_cell: &MazeCell) -> f32 {
    if angle > 0.0 && angle < 90.0 {
        find_sensor_value(x_in_cell, y_in_cell, angle, maze_cell)
    } else if angle >= 90.0 && angle < 180 {
        find_sensor_value(x_in_cell, y_in_cell, angle, maze_cell)
    } else {
        1.0
    }
}

pub fn sensor_will_hit_wall_in_cell(x_in_cell: f32, y_in_cell: f32, angle: f32, maze_cell: &MazeCell) -> bool {
    false
}

pub fn get_sensor_values_from_position(current_x: u32, current_y: u32, maze: &MazePhenotype) -> [u32; 6] {


    // find out where sensors will intersect walls
    // create line from current position to first wall encounter

    // calculate distance between current position and intersections

    // return list of all values
    let xs: [u32; 6] = [1, 2, 3, 4, 5, 6];
    xs
}

pub fn simulate_navigator_in_maze(navigator: &Navigator, maze: &MazePhenotype) {

    // initiate position
    //let mut x: u32 = CELL_TO_UNITS * 0.5;
    //let mut y: u32 = CELL_TO_UNITS * 0.5;

    //repeat

    // find sensor values from position
    // send values to navigator model
    // get movement direction
    // validate movement
    // update navigator position in maze
    // check if finished
    // check if time is up or maze steps have been taken

    let mut steps_left = 100;
    let mut maze_completed = false;

    while steps_left > 0 && !maze_completed {
        //let sensor_values = get_sensor_values_from_position(x, y, maze);

        /*let movement = navigator.get_movement(sensor_values);

        let new_position = get_new_positions(x, y, movement);

        x = new_position.x;
        y = new_position.y;*/


        steps_left -= 1;
    }
}
