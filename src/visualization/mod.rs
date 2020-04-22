pub mod maze;
pub mod simulation;

#[derive(Debug, Clone)]
pub struct VisualizationOptions {
    pub folder_path: String,
    pub file_name: String,
    pub save_all_steps: bool,
}
