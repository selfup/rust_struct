use std::cell::Cell;

struct Coordinates {
    x: Cell<i32>,
    y: Cell<i32>,
}

fn main() {
    let bike = Coordinates { x: Cell::new(0), y: Cell::new(0) };
    let mut previous_positions = vec![];

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
    v_cc.push(x.get());
    v_cc.push(y.get());
    v_cc
}

fn move_up(y_cord: &Cell<i32>) -> &Cell<i32> {
    y_cord.set(y_cord.get() + 1);
    y_cord
}

fn move_down(y_cord: &Cell<i32>) -> &Cell<i32> {
    y_cord.set(y_cord.get() - 1);
    y_cord
}

fn move_right(x_cord: &Cell<i32>) -> &Cell<i32> {
    x_cord.set(x_cord.get() + 1);
    x_cord
}

fn move_left(x_cord: &Cell<i32>) -> &Cell<i32> {
    x_cord.set(x_cord.get() - 1);
    x_cord
}

#[test]
fn it_can_move() {
    let bike = Coordinates { x: Cell::new(0), y: Cell::new(0) };
    let mut previous_positions = vec![];

    // test it starts at 0

    assert_eq!(0, bike.y.get());

    // test it can move up one and can add positions to the vector

    move_up(&bike.y);
    add_positions(&mut previous_positions, &bike.y, &bike.x);

    assert_eq!(1, bike.y.get());
    assert_eq!(0, previous_positions[0]);
    assert_eq!(1, previous_positions[1]);

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
