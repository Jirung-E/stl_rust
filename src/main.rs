fn main() {
    let mut a = (1..=100).collect::<Vec<_>>();

    a.iter().for_each(|x| println!("{}", x));
    println!("sum: {}", a.iter().sum::<i32>());
}