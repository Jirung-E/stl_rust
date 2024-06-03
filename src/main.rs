fn main() {
    let v: [i32; 5] = [1, 3, 5, 7, 9];

    if v.iter().all(|x| { x & 1 == 1 }) {
        println!("모두 홀수");
    }
    else {
        println!("모두 홀수인 것은 아니다.");
    }
}