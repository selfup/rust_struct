
extern crate rust_struct;
extern crate diesel;

use self::rust_struct::*;
use self::rust_struct::models::*;
use self::diesel::prelude::*;

fn main() {
    use rust_struct::schema::previous_positions::dsl::*;

    let connection = establish_connection();
    let results = previous_positions.filter(dead.eq(true))
        .limit(5)
        .load::<PreviousPosition>(&connection)
        .expect("Error loading previous_positions");

    println!("Displaying {} previous_positions", results.len());
    for previous_position in results {
        println!("{}", previous_position.position);
        println!("----------\n");
        println!("{}", previous_position.dead);
    }
}
