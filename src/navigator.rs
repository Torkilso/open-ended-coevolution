pub struct Navigator {
    test: i32,
}

pub struct Species {
    population: Vec<Navigator>
}

impl Navigator {
    pub fn new() -> Navigator {
        Navigator {
            test: 0
        }
    }
}
