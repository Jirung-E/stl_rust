use std::iter;

fn main() {
    iter::successors(Some(0), |i| Some(i + 1))
        .take(10)
        .for_each(|i| {
            println!("{}", i);
        });
}