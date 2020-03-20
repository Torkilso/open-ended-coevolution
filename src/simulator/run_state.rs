use crate::config;
use crate::maze::maze_phenotype::MazePhenotype;
use crate::simulator::sensor::{
    find_sensor_value_north_east, find_sensor_value_north_west, find_sensor_value_south_east,
    find_sensor_value_south_west,
};
use crate::simulator::Point;
use std::f64::consts::PI;

pub struct RunState {
    pub(crate) global_x: f64,
    pub(crate) global_y: f64,
    current_cell_x: u32,
    current_cell_y: u32,
    current_x_in_cell: f64,
    current_y_in_cell: f64,
    pub(crate) current_direction: f64,
    pub(crate) current_velocity: f64,
    pub(crate) current_angular_velocity: f64,
}

impl RunState {
    pub fn new(height: u32) -> RunState {
        RunState {
            global_y: height as f64 - 0.5,
            global_x: 0.5,
            current_cell_x: 0,
            current_cell_y: height - 1,
            current_x_in_cell: 0.5,
            current_y_in_cell: 0.5,
            current_direction: config::AGENT.start_offset,
            current_velocity: 0.0,
            current_angular_velocity: 0.0,
        }
    }

    pub fn update_velocities(
        &mut self,
        velocity_adjustment: f64,
        angular_velocity_adjustment: f64,
    ) {
        self.current_velocity += (velocity_adjustment); // - 0.5);
        self.current_angular_velocity += (angular_velocity_adjustment); // - 0.5);

        // constraints of speed & angular velocity
        if self.current_velocity > config::AGENT.max_speed {
            self.current_velocity = config::AGENT.max_speed;
        };
        if self.current_velocity < -config::AGENT.max_speed {
            self.current_velocity = -config::AGENT.max_speed;
        };
        if self.current_angular_velocity > config::AGENT.max_speed {
            self.current_angular_velocity = config::AGENT.max_speed;
        };
        if self.current_angular_velocity < -config::AGENT.max_speed {
            self.current_angular_velocity = -config::AGENT.max_speed;
        };
    }

    pub fn update_position(&mut self, maze: &MazePhenotype) -> Point {
        // get horizontal and vertical velocity components
        let vx = ((self.current_direction / 180.0 * PI).cos() * self.current_velocity)
            / config::MAZE.cell_dimension;
        let vy = ((self.current_direction / 180.0 * PI).sin() * self.current_velocity)
            / config::MAZE.cell_dimension;

        // Update agent heading
        self.current_direction += self.current_angular_velocity;

        if self.current_direction > 360.0 {
            self.current_direction -= 360.0
        }
        if self.current_direction < 0.0 {
            self.current_direction += 360.0
        }

        let mut new_global_x = self.global_x + vx;
        let mut new_global_y = self.global_y + vy;

        if new_global_x < 0.0 {
            new_global_x = 0.0;
        }
        if new_global_y < 0.0 {
            new_global_y = 0.0;
        }

        let new_current_x_in_cell = new_global_x % 1.0;
        let new_current_y_in_cell = new_global_y % 1.0;

        if self.will_collide_with_wall(
            new_global_x.floor() as u32,
            new_global_y.floor() as u32,
            new_current_x_in_cell,
            new_current_y_in_cell,
            maze,
        ) {
            return Point::new(self.global_x, self.global_y); // return same position as last time step
        }

        // Find next agent's location
        self.global_x = new_global_x;
        self.global_y = new_global_y;

        self.current_cell_x = new_global_x.floor() as u32;
        self.current_cell_y = new_global_y.floor() as u32;

        self.current_x_in_cell = new_current_x_in_cell;
        self.current_y_in_cell = new_current_y_in_cell;

        Point::new(self.global_x, self.global_y)
    }

    fn will_collide_with_wall(
        &self,
        new_current_cell_x: u32,
        new_current_cell_y: u32,
        new_x_in_cell: f64,
        new_y_in_cell: f64,
        maze: &MazePhenotype,
    ) -> bool {
        let cell = maze.get_cell_at(new_current_cell_x, new_current_cell_y);

        let scaled_x = new_x_in_cell * config::MAZE.cell_dimension;
        let scaled_y = new_y_in_cell * config::MAZE.cell_dimension;

        // if x is close to top
        if cell.north_wall && config::MAZE.cell_dimension - scaled_y < config::AGENT.agent_radius {
            return true;
        }
        if cell.south_wall && scaled_y < config::AGENT.agent_radius {
            return true;
        }
        if cell.east_wall && config::MAZE.cell_dimension - scaled_x < config::AGENT.agent_radius {
            return true;
        }
        if cell.west_wall && scaled_x < config::AGENT.agent_radius {
            return true;
        }
        false
    }

    pub fn maze_completed(&self, width: u32) -> bool {
        self.global_x > width as f64 - 0.5 && self.global_y < 0.5
    }

    pub fn get_sensor_value(&self, angle: f64, maze: &MazePhenotype) -> f64 {
        if angle >= 0.0 && angle < 90.0 {
            find_sensor_value_north_east(
                angle,
                self.current_x_in_cell,
                self.current_y_in_cell,
                self.current_cell_x,
                self.current_cell_y,
                maze,
            )
        } else if angle >= 90.0 && angle < 180.0 {
            find_sensor_value_north_west(
                angle,
                self.current_x_in_cell,
                self.current_y_in_cell,
                self.current_cell_x,
                self.current_cell_y,
                maze,
            )
        } else if angle >= 180.0 && angle < 270.0 {
            find_sensor_value_south_west(
                angle,
                self.current_x_in_cell,
                self.current_y_in_cell,
                self.current_cell_x,
                self.current_cell_y,
                maze,
            )
        } else if angle >= 270.0 && angle < 360.0 {
            find_sensor_value_south_east(
                angle,
                self.current_x_in_cell,
                self.current_y_in_cell,
                self.current_cell_x,
                self.current_cell_y,
                maze,
            )
        } else {
            1.0
        }
    }

    pub fn get_all_sensor_values(&self, maze: &MazePhenotype) -> Vec<f64> {
        let mut sensor_values: Vec<f64> = Vec::new();

        for angle in SENSOR_BASE_ANGLES {
            let value = self.get_sensor_value((angle + self.current_direction) % 360.0, maze);
            sensor_values.push(value);
        }
        sensor_values
    }
}

static SENSOR_BASE_ANGLES: &'static [f64] = &[0.0, 45.0, 90.0, 180.0, 270.0, 315.0];
