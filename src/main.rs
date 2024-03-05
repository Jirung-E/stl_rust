// [문제] main을 수정하지 않고 의도대로 실행되게
// change함수를 선언하고 정의하라


fn main() {
    let mut a = Dog { data: 1 };
    let mut b = Dog { data: 2 };

    println!("{:?}, {:?}", a, b);

    change(&mut a, &mut b);

    println!("{:?}, {:?}", a, b);
}


#[derive(Debug)]
struct Dog {
    data: i32
}

impl Clone for Dog {
    fn clone(&self) -> Dog {
        Dog {
            data: self.data
        }
    }
}

fn change<T: Clone>(a: &mut T, b: &mut T) {
    let temp = a.clone();
    *a = b.clone();
    *b = temp;
}