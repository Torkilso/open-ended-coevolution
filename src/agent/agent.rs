pub struct Agent {
    pub genome: u32,
    species_id: u32,
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            genome: 0,
            species_id: 0,
        }
    }
}
