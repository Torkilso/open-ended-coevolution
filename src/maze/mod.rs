pub mod maze_genotype;
pub mod maze_phenotype;
pub mod maze_validator;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum PathDirection {
    North,
    East,
    South,
    West,
    None,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpeningLocation {
    North,
    East,
    South,
    West,
}
