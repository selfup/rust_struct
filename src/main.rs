use std::cell::Cell;

struct Coordinates {
    x: Cell<i32>,
    y: Cell<i32>,
}

fn main() {
    let origin = Coordinates { x: Cell::new(0), y: Cell::new(0) };

    move_up(&origin.y); // borrow the value
    move_up(&origin.y); // so that it can change

    println!("X is {:?} and Y is {:?}", origin.x, origin.y);
}

fn move_up(y_cord: &Cell<i32>) -> &Cell<i32> {
    let new_val = y_cord.get(); // get the value and store it
    y_cord.set(new_val + 1); // modify the stored value and set the incoming param
    y_cord // no semicolon here because it is the return value
}

fn move_down(y_cord: &Cell<i32>) -> &Cell<i32> {
    let new_val = y_cord.get(); // get the value and store it
    y_cord.set(new_val - 1); // modify the stored value and set the incoming param
    y_cord // no semicolon here because it is the return value
}

#[test]
fn it_can_move() {
    let origin = Coordinates { x: Cell::new(0), y: Cell::new(0) };

    assert_eq!(0, origin.y.get());

    move_up(&origin.y);

    assert_eq!(1, origin.y.get());

    move_up(&origin.y);
    move_up(&origin.y);
    move_up(&origin.y);

    assert_eq!(4, origin.y.get());

    for _ in 0..19 { // _ instead of x so that rust does not complain about x not being used
        move_up(&origin.y); // moves up 19 times
    }

    assert_eq!(23, origin.y.get());

    for _ in 0..19 {
        move_down(&origin.y);
    }

    assert_eq!(4, origin.y.get());
}
