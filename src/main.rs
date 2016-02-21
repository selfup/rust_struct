use std::cell::Cell;

struct Coordinates {
    x: Cell<i32>,
    y: Cell<i32>,
}

fn main() {
    let origin = Coordinates { x: Cell::new(0), y: Cell::new(0) };

    println!("X is {:?} and Y is {:?}", origin.x, origin.y);

    move_up(&origin.y); // borrowing the value
    move_up(&origin.y); // so that I can change it

    println!("X is {:?} and Y is {:?}", origin.x, origin.y);
}

fn move_up(y_cord: &Cell<i32>) -> &Cell<i32> {
    let new_val = y_cord.get(); // Here I get the value and store it
    y_cord.set(new_val + 1); // Here I modify the stored value and set the incoming param
    y_cord // no semicolon here because it is the return value
}

#[test]
fn it_can_move_up_one_pixel() {
    let origin = Coordinates { x: Cell::new(0), y: Cell::new(0) };
    let origin_test = Coordinates { x: Cell::new(1), y: Cell::new(0) };

    assert_eq!(origin_test.y, origin.y);

    move_up(&origin.y);

    assert_eq!(1, origin.y.get());

    move_up(&origin.y);
    move_up(&origin.y);
    move_up(&origin.y);

    assert_eq!(4, origin.y.get())
}
