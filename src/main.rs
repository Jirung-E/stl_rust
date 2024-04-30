fn main() {
    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7, 8];

    let v = [a, b].concat();
    println!("{:?}", v);
}