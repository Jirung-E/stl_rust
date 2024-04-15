use std::fs;

fn main() {
    // [문제] "stl.cpp"에 있는 알파벳 소문자의 출현 횟수를 다음과 같이 출력하라
    // a - 20
    // b - 3
    // c - 1
    // ...
    // z - 2

    let file = fs::read_to_string("src/main.rs")
        .expect("파일을 읽을 수 없습니다.");

    let mut counts = vec![0; 'z' as usize - 'a' as usize + 1];

    file.chars()
        .filter(|c| c.is_lowercase())
        .for_each(|c| counts[c as usize - 'a' as usize] += 1);

    for (i, count) in counts.iter().enumerate() {
        println!("{} - {}", (i as u8 + 'a' as u8) as char, count);
    }
}