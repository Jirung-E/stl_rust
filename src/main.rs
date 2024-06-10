use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let mut v: Vec<Dog> = Vec::from_iter((0..100).map(|_| Dog::new()));

    for dog in &v {
        println!("{} --- {}", dog.n, dog.c);
    }
    println!();
    println!();

    v.sort_by_key(|dog| dog.n);

    for dog in &v {
        println!("{} --- {}", dog.n, dog.c);
    }
    println!();
    println!();
}


struct Dog {
    n: i32,
    c: char,
}

impl Dog {
    fn new() -> Dog {
        Dog {
            n: rand::thread_rng().gen_range(1..10),
            c: rand::thread_rng().gen_range('a'..='z'),
        }
    }
}