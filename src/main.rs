use rand::Rng;

fn main() {
    let d1 = (0..100)
        .map(|_| rand::thread_rng().gen_range(0..100))
        .collect::<Vec<i32>>();
    println!("{:?}", d1);

    let d2 = (0..10)
        .map(|_| {
            (0..10)
                .map(|_| rand::thread_rng().gen_range(0..100))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    for i in d2.iter() {
        for k in i.iter() {
            print!("{:4}", k);
        }
        println!();
    }
}