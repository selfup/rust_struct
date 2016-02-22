use std::cell::Cell;

struct Coordinates {
    x: Cell<i32>,
    y: Cell<i32>,
}

fn main() {
    let origin = Coordinates { x: Cell::new(0), y: Cell::new(0) };

    // borrow the value so that it can change (using the '&' symbol borrows the var)
    move_up(&origin.y);
    move_up(&origin.y);
    move_down(&origin.y);
    move_right(&origin.x);
    move_right(&origin.x);
    move_left(&origin.x);

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

fn move_right(x_cord: &Cell<i32>) -> &Cell<i32> {
    let new_val = x_cord.get(); // get the value and store it
    x_cord.set(new_val + 1); // modify the stored value and set the incoming param
    x_cord // no semicolon here because it is the return value
}

fn move_left(x_cord: &Cell<i32>) -> &Cell<i32> {
    let new_val = x_cord.get(); // get the value and store it
    x_cord.set(new_val - 1); // modify the stored value and set the incoming param
    x_cord // no semicolon here because it is the return value
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

    // test it can move down

    for _ in 0..19 {
        move_down(&origin.y);
    }

    assert_eq!(4, origin.y.get());

    // test it can move right

    for _ in 0..19 {
        move_right(&origin.x);
    }

    assert_eq!(19, origin.x.get());

    // test it can move left

    for _ in 0..10 {
        move_left(&origin.x);
    }

    assert_eq!(9, origin.x.get());
}
