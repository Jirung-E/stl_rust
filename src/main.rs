// [문제] 가장 큰 수를 찾아 화면에 출력한다.


use rand::Rng;

fn main() {
    let mut a: [i32;1000] = [0;1000];
    for num in a.iter_mut() {
        *num = rand::thread_rng().gen_range(1000..9999);
        print!("{:8}", num);
    }


    println!("\n{}", a.iter().max().unwrap());
}