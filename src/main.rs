use rand::seq::SliceRandom;

fn main() {
    let mut v: Vec<i32> = (0..100).collect();

    for num in v.iter() {
        print!("{:4}", num);
    }
    println!();
    println!();

    v.shuffle(&mut rand::thread_rng());

    for num in v.iter() {
        print!("{:4}", num);
    }
    println!();
    println!();

    v.sort();

    for num in v.iter() {
        print!("{:4}", num);
    }
}
