#[derive(Queryable)]
pub struct PreviousPosition {
    pub id: i32,
    pub position: String,
    pub dead: bool,
}

pub struct Coordinate<'a> {
    pub x: &'a i32,
    pub y: &'a i32,
}
