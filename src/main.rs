struct Coordinates {
    x: i32,
    y: i32,
}

fn main() {
    let mut origin = Coordinates { x: 0, y: 0 };

    println!("X is {} and Y is {})", origin.x + 1, origin.y + 90);

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}
