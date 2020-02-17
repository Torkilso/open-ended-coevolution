use crate::navigator::Navigator;
use crate::maze_phenotype::MazePhenotype;

pub static CELL_TO_UNITS: u32 = 20;

pub fn get_sensor_values_from_position(current_x: u32, current_y: u32, maze: &MazePhenotype) -> [u32; 6] {


    // find out where sensors will intersect walls
    // calculate distance between current position and intersections

    // return list of all values
    let xs: [u32; 6] = [1, 2, 3, 4, 5, 6];
    xs
}

pub fn simulate_navigator_in_maze(navigator: &Navigator, maze: &MazePhenotype) {

    // initiate position
    let mut x: u32 = CELL_TO_UNITS * 0.5;
    let mut y: u32 = CELL_TO_UNITS * 0.5;

    //repeat

    // find sensor values from position
    // send values to navigator model
    // get movement direction
    // validate movement
    // update navigator position in maze
    // check if finished
    // check if time is up or maze steps have been taken

    let mut steps_left = 100;

    while time_left > 0 {
        let sensor_values = get_sensor_values_from_position(x, y, maze);

        /*let movement = navigator.get_movement(sensor_values);

        let new_position = get_new_positions(x, y, movement);

        x = new_position.x;
        y = new_position.y;*/


        steps_left -= 1;
    }
}
