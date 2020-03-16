use crate::neatns::agent::Agent;
use crate::maze::maze_phenotype::{MazeCell, MazePhenotype};
use crate::simulator::run_state::RunState;

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
                current_y_in_cell + missing_side,
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
                current_cell_y + 1,
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
                current_cell_y + 1,
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
                current_cell_y - 1,
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
                current_cell_y - 1,
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
