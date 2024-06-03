fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let (even, odd): (Vec<_>, Vec<_>) = v
        .into_iter()
        .partition(|n| n % 2 == 0);

    println!("{:?}", even);
    println!("{:?}", odd);
}
