#[derive(Queryable)]
pub struct PreviousPosition {
    pub id: i32,
    pub position: String,
    pub dead: bool,
}

pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}
