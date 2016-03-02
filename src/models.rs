#[derive(Queryable)]
pub struct PreviousPosition {
    pub id: i32,
    pub position: String,
    pub dead: bool,
}

pub struct Coordinate {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}
