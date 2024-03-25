use std::io::Write;

fn main() {
    loop {
        print!("숫자를 입력하라 - ");
        std::io::stdout().flush().unwrap();
        let num = input().trim().parse::<u32>().unwrap();
        if num == 0 {
            break;
        }

        // u128로 sum
        // 게으른 반복자 덕분에 u128 * num 만큼의 메모리를 잡아먹진 않는다.
        let sum: u128 = (1..=num).map(|x| x as u128).sum();
        // sum::<u128>이 안된다...
        println!("1부터 {}까지의 합은 {}", num, sum);

        input();
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .unwrap();
    buf
}