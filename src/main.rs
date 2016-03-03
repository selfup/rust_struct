use std::cell::Cell;

struct Coordinates {
    x: Cell<i32>,
    y: Cell<i32>,
}

fn main() {
    let bike = Coordinates { x: Cell::new(0), y: Cell::new(0) };
    let mut previous_positions = vec![];
    // borrow the value so that it can change (using the '&' symbol borrows the var)
    move_up(&bike.y);
    add_positions(&mut previous_positions, &bike.y, &bike.x);
    move_down(&bike.y);
    add_positions(&mut previous_positions, &bike.y, &bike.x);
    move_right(&bike.x);
    add_positions(&mut previous_positions, &bike.y, &bike.x);
    move_left(&bike.x);
    add_positions(&mut previous_positions, &bike.y, &bike.x);

    println!("X is {:?}, Y is {:?}", bike.x, bike.y);
    println!("{:?}", previous_positions)

}

fn add_positions<'a>(v_cc: &'a mut Vec<i32>, y: &Cell<i32>, x: &Cell<i32>) -> &'a mut Vec<i32> {
    let c_y = y;
    let c_x = x;
    v_cc.push(c_x.get());
    v_cc.push(c_y.get());
    v_cc
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
    let bike = Coordinates { x: Cell::new(0), y: Cell::new(0) };

    // test it starts at 0

    assert_eq!(0, bike.y.get());

    // test it can move up one

    move_up(&bike.y);

    assert_eq!(1, bike.y.get());

    // test it can continue to move up and theat move logic is valid

    move_up(&bike.y);
    move_up(&bike.y);
    move_up(&bike.y);

    assert_eq!(4, bike.y.get());

    for _ in 0..19 { // _ instead of x so that rust does not complain about x not being used
        move_up(&bike.y); // moves up 19 times
    }

    assert_eq!(23, bike.y.get());

    // test it can move down

    for _ in 0..19 {
        move_down(&bike.y);
    }

    assert_eq!(4, bike.y.get());

    // test it can move right

    for _ in 0..19 {
        move_right(&bike.x);
    }

    assert_eq!(19, bike.x.get());

    // test it can move left

    for _ in 0..10 {
        move_left(&bike.x);
    }

    assert_eq!(9, bike.x.get());
}
