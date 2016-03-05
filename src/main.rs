mod collision;

use std::cell::Cell;

struct Coordinates {
    x: Cell<i32>,
    y: Cell<i32>,
}

fn main() {
    let bike_one = Coordinates { x: Cell::new(0), y: Cell::new(50) };
    let bike_two = Coordinates { x: Cell::new(0), y: Cell::new(50) };
    let mut one_pos = vec![];
    let mut two_pos = vec![];
    let mut all_pos = vec![];
    let mut bike_stats = vec![];

    move_up(&bike_one.y);
    move_down(&bike_two.y);
    add_positions(&mut one_pos, &bike_one.y, &bike_one.x);
    add_positions(&mut two_pos, &bike_two.y, &bike_two.x);
    combined_positions(&mut all_pos, &mut one_pos, &mut two_pos);
    collision::alive_or_dead(&mut one_pos, &mut two_pos, &mut bike_stats);

    move_right(&bike_one.x);
    move_left(&bike_two.x);
    add_positions(&mut one_pos, &bike_one.y, &bike_one.x);
    add_positions(&mut two_pos, &bike_two.y, &bike_two.x);
    combined_positions(&mut all_pos, &mut one_pos, &mut two_pos);
    collision::alive_or_dead(&mut one_pos, &mut two_pos, &mut bike_stats);

    println!("Bike One: X is {:?}, Y is {:?}\nPositions: {:?}\n", bike_one.x, bike_one.y, one_pos);
    println!("Bike Two: X is {:?}, Y is {:?}\nPositions: {:?}\n", bike_two.x, bike_two.y, two_pos);
    println!("Combined positions are: {:?}", all_pos)
}

fn add_positions<'a>(v_cc: &'a mut Vec<i32>, y: &Cell<i32>, x: &Cell<i32>) -> &'a mut Vec<i32> {
    v_cc.push(x.get());
    v_cc.push(y.get());
    v_cc
}

fn combined_positions<'a>(a_p: &'a mut Vec<i32>, o_p: &mut Vec<i32>, t_p: &mut Vec<i32>) -> &'a mut Vec<i32> {
    a_p.extend_from_slice(o_p);
    a_p.extend_from_slice(t_p);
    a_p
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
    let bike_one = Coordinates { x: Cell::new(0), y: Cell::new(0) };
    assert_eq!(0, bike_one.y.get());

    move_up(&bike_one.y);
    move_up(&bike_one.y);
    move_up(&bike_one.y);
    move_up(&bike_one.y);
    assert_eq!(4, bike_one.y.get());

    for _ in 0..19 {
        move_up(&bike_one.y);
    }
    assert_eq!(23, bike_one.y.get());

    for _ in 0..19 {
        move_down(&bike_one.y);
    }
    assert_eq!(4, bike_one.y.get());

    for _ in 0..19 {
        move_right(&bike_one.x);
    }
    assert_eq!(19, bike_one.x.get());

    for _ in 0..10 {
        move_left(&bike_one.x);
    }
    assert_eq!(9, bike_one.x.get());
}

#[test]
fn it_can_log_all_positions() {
    let bike_one = Coordinates { x: Cell::new(0), y: Cell::new(50) };
    let bike_two = Coordinates { x: Cell::new(100), y: Cell::new(50) };
    let mut one_pos = vec![];
    let mut two_pos = vec![];
    let mut all_pos = vec![];

    move_up(&bike_one.y);
    move_up(&bike_two.y);
    add_positions(&mut one_pos, &bike_one.y, &bike_one.x);
    assert_eq!(51, bike_one.x.get() + bike_one.y.get());

    add_positions(&mut two_pos, &bike_two.y, &bike_two.x);
    assert_eq!(151, bike_two.x.get() + bike_two.y.get());

    combined_positions(&mut all_pos, &mut one_pos, &mut two_pos);
    let sum = all_pos.iter().fold(0, |acc, &x| acc + x);
    assert_eq!(202, sum);
}
