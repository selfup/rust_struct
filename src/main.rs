struct Coordinates {
    x: i32,
    y: i32,
}

fn main() {
    let origin = Coordinates { x: 0, y: 0 };

    println!("X is {} and Y is {}", origin.x + 1, origin.y + 90);

    let mut v = vec![1, 2];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    println!("X is {} and Y is {}", move_right(origin.x), move_up(origin.y));
    println!("X is {} and Y is {}", move_right(origin.x), move_up(origin.y));
}

fn move_up(y_cord: i32) -> i32 {
    y_cord - 1
}

fn move_right(x_cord: i32) -> i32 {
    x_cord + 1
}
