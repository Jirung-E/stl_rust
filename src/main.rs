use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    find_max();
}

fn make_file() {
    let mut file = match File::create("random_numbers.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating the file: {:?}", error)
        },
    };

    let mut a = [0;1000];

    for num in a.iter_mut() {
        *num = rand::thread_rng().gen_range(0..=99_999);
        file.write(format!("{}\n", num).as_bytes()).unwrap();
    }
}

fn find_max() {
    let mut file = match File::open("random_numbers.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    println!("{}",
        buf.lines()
            .map(|x| { x.parse::<i32>().unwrap() })
            .collect::<Vec<i32>>().iter()
            .max().unwrap()
    );
}
