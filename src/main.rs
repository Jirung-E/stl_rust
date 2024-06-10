use std::iter;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let v: Vec<_> = iter::successors(Some(1), |i| Some(i + 1))
        .skip(10000)      // cpp::views::drop
        .take(1_000_000)       // cpp::views::take
        .collect();      // cpp::ranges::to<vector>
    println!("Time: {:?} micro seconds", start.elapsed().as_micros());

    // for num in v {
    //     print!("{} ", num);
    // }
}