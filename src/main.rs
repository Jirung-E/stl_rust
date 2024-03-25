use std::io::Write;

fn main() {
    loop {
        print!("숫자를 입력하라 - ");
        std::io::stdout().flush().unwrap();
        let num = input().trim().parse::<u32>().unwrap();
        if num == 0 {
            break;
        }

        let v: Vec<u32> = (1..=num).collect();

        // println!("{:?}", v);

        // u128로 sum
        let sum: u128 = v.iter().map(|x| *x as u128).sum();
        // sum::<u128>이 안된다...
        println!("1부터 {}까지의 합은 {}", num, sum);
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .unwrap();
    buf
}