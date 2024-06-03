fn main() {
    let v: [i32; 0] = [];

    if v.iter().all(|x| { x & 1 == 1 }) {
        println!("홀수가 아닌것은 한개도 없다.");
    }
    else {
        println!("홀수가 아닌게 있다.");
    }
}