use crate::maze::maze_phenotype::MazePhenotype;
use crate::config;
use crate::simulator::run_state::RunState;

#[derive(Debug, Clone)]
pub struct RadarValues {
    forward: bool,
    right: bool,
    back: bool,
    left: bool,
}

impl RadarValues {
    pub fn new() -> RadarValues {
        RadarValues {
            forward: false,
            right: false,
            back: false,
            left: false,
        }
    }

    pub fn to_f64_vector(&self) -> Vec<f64> {
        vec![if self.forward { 1.0 } else { 0.0 }, if self.right { 1.0 } else { 0.0 }, if self.back { 1.0 } else { 0.0 }, if self.left { 1.0 } else { 0.0 }]
    }
}

pub fn get_radar_values(run_state: &RunState, maze: &MazePhenotype) -> RadarValues {
    let mut radar_values = RadarValues::new();

    let agent_x = run_state.global_x();
    let agent_y = run_state.global_y();

    let mut angle = 0.0;

    if agent_x <= maze.width as f64 - 0.5 && agent_y >= 0.5 {
        angle = 270f64 + ((maze.width as f64 - 0.5 - agent_x) / (agent_y - 0.5)).atan().to_degrees();
    } else if agent_x <= maze.width as f64 - 0.5 && agent_y <= 0.5 {
        // below y goal
        angle = ((0.5 - agent_y) / (maze.width as f64 - 0.5 - agent_x)).atan().to_degrees();
    } else if agent_x >= maze.width as f64 - 0.5 && agent_y >= 0.5 {
        // right of x goal
        angle = 270f64 - ((maze.width as f64 - agent_x) / (agent_y - 0.5)).atan().to_degrees();

    }
    let mut difference = run_state.current_direction - angle;

    if difference == 0.0 {
        radar_values.forward = true;
        return radar_values;
    } else if difference < 0.0 {
        difference *= -1.0;
    } else {
        difference = 360.0 - difference;
    }

    if difference >= 0.0 && difference < 45.0 {
        radar_values.forward = true;
    } else if difference >= 45.0 && difference < 135.0 {
        radar_values.left = true;
    } else if difference >= 135.0 && difference < 225.0 {
        radar_values.back = true;
    } else if difference >= 225.0 && difference < 315.0 {
        radar_values.right = true;
    } else if difference >= 315.0 && difference < 360.0 {
        radar_values.forward = true;
    }

    radar_values
}

