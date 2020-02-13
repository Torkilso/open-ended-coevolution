#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PathDirection {
    North,
    East,
    South,
    West,
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpeningLocation {
    North,
    East,
    South,
    West,
}
