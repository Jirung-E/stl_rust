fn main() {
    let mut a = [0;100];

    for (i, num) in a.iter_mut().enumerate() {
        *num += i;
    }

    for num in a.iter() {
        print!("{:8}", num);
    }
}