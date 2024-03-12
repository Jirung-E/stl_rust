fn main() {
    let mut a = [0;100];

    a.iter_mut().enumerate().for_each(|(i, num)| *num = i+1);

    for num in a.iter() {
        print!("{:8}", num);
    }
}