use crate::maze::maze_phenotype::MazePhenotype;
use std::f64::consts::PI;
use crate::simulator::sensor::{find_sensor_value_south_east, find_sensor_value_south_west, find_sensor_value_north_west, find_sensor_value_north_east};
use crate::config;

pub struct RunState {
    global_x: f64,
    global_y: f64,
    pub(crate) current_direction: f64,
    current_velocity: f64,
    current_angular_velocity: f64,
}

impl RunState {
    pub fn new(height: u32) -> RunState {
        RunState {
            global_y: height as f64 - 0.5,
            global_x: 0.5,
            current_direction: config::AGENT.start_offset,
            current_velocity: 0.0,
            current_angular_velocity: 0.0,
        }
    }

    pub fn current_x_cell(&self) -> u32 {
        self.global_x as u32
    }

    pub fn current_y_cell(&self) -> u32 {
        self.global_y as u32
    }

    pub fn current_x_in_cell(&self) -> f64 {
        self.global_x % 1.0
    }

    pub fn current_y_in_cell(&self) -> f64 {
        self.global_y % 1.0
    }

    pub fn update_velocities(&mut self, velocity_adjustment: f64, angular_velocity_adjustment: f64) {
        self.current_velocity += (velocity_adjustment);// - 0.5);
        self.current_angular_velocity = (angular_velocity_adjustment);// - 0.5);

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

    pub fn update_position(&mut self, maze: &MazePhenotype) {
        // get horizontal and vertical velocity components
        let vx = (self.current_direction / 180.0 * PI).cos() * self.current_velocity;
        let vy = (self.current_direction / 180.0 * PI).sin() * self.current_velocity;

        // Update agent heading
        self.current_direction += self.current_angular_velocity;

        if self.current_direction > 360.0 {
            self.current_direction -= 360.0
        }
        if self.current_direction < 0.0 {
            self.current_direction += 360.0
        }

        let mut new_global_x = self.global_x() + vx;
        let mut new_global_y = self.global_y() + vy;

        if new_global_x < 0.0 {
            new_global_x = 0.0;
        }

        if new_global_y < 0.0 {
            new_global_y = 0.0;
        }

        if self.will_collide_with_wall(new_global_x, new_global_y, maze) {
            return;
        }

        // Find next agent's location
        self.current_cell_x = new_global_x.floor() as u32;
        self.current_cell_y = new_global_y.floor() as u32;

        self.current_x_in_cell = new_global_x % 1.0;
        self.current_y_in_cell = new_global_y % 1.0;
    }

    fn will_collide_with_wall(&self, new_x: f64, new_y: f64, maze: &MazePhenotype) -> bool {
        let cell = maze.get_cell_at(new_x.floor() as u32, new_y.floor() as u32);

        // if x is close to top
        if cell.north_wall && new_y % 1.0 < config::AGENT.agent_radius {
            return true;
        }
        if cell.south_wall && new_y % 1.0 < config::AGENT.agent_radius {
            return true;
        }
        if cell.east_wall && new_x % 1.0 < config::AGENT.agent_radius {
            return true;
        }
        if cell.west_wall && new_x % 1.0 < config::AGENT.agent_radius {
            return true;
        }
        false
    }

    pub fn maze_completed(&self, width: u32, height: u32) -> bool {
        self.current_cell_x == width - 1 && self.current_cell_y == height - 1 && self.current_x_in_cell >= 0.5 && self.current_y_in_cell >= 0.5
    }

    pub fn distance_from_goal(&self) -> f64 {
        0.0
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
        let sensor_base_angles: Vec<f64> = vec![0.0, 45.0, 90.0, 180.0, 270.0, 315.0];

        let mut sensor_values: Vec<f64> = Vec::new();

        for angle in sensor_base_angles {
            let value = self.get_sensor_value((angle + self.current_direction) % 360.0, maze);
            sensor_values.push(value);
        }
        sensor_values
    }
}