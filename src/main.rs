
fn main() {
    println!("{}", add(1, 2));
    println!("{}", add(1.1, 2.2));
    // println!("{}", add("Hello", " World"));
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

