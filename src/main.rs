pub mod field;

use std::{io, thread, time};

use field::Field;

fn main() {
    let mut user_input = String::new();
    
    println!("Enter coordinates to place living cells: ");
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {},
        Err(error) => panic!("Problem reading the line: {:?}", error),
    }

    let input_values: Vec<&str> = user_input.split(" ").collect();

    let mut field = Field::init(10);

    for value_pair in input_values {
        let values: Vec<&str> = value_pair.split("-").collect();
        let x: usize = values.first().unwrap().parse().unwrap();
        let y: usize;
        let mut y_str = values.last().unwrap().to_string();
        if y_str.ends_with("\r\n") {
            y_str.truncate(y_str.len() - 2);
            y = y_str.parse().unwrap();
        } else {
            y = y_str.parse().unwrap();
        }


        println!("{:?}, {:?}", x, y);
        field.make_alive(x, y);
    }
    field.print();

    loop {
        field.step();
        field.print();
        thread::sleep(time::Duration::from_millis(500));
    }
}
