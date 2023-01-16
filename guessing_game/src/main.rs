extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // let으로 변수선언
        // mut를 붙이면 가변변수가 된다.
        // 러스트에서 변수는 기본적으로 불변

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        /*  use std::io 문이 없다면 std::io::stdin()으로 사용해야한다.
     *  $는 참조자, 데이터를 여러번 복사하지 않도록 해준다.
     *  참조자도 불변이므로 mut를 붙여서 가변으로 만들어줘야한다.
     *  read_line()은 io::Result라는 값을 돌려준다.
     *  열거형으로 Ok, Err 값을 가진다.
     *  Err인 경우 .expect()는 프로그램 작동을 멈추고 메시지를 출력하도록 한다.
     */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Ok 가 나오면 num을 돌려준다.
            Err(_)  => continue
            // _ 언더바는 모든 값과 매칭할 수 있다.
        };
        // 앞서 이미 guess라는 변수가 선언되었지만 러스트는 섀도우라는 이전 값을 가리는 것을 지원한다.
        // 다른 변수를 만드는 대신 재사용할 수 있다.
        // trim()은 앞뒤 빈칸을 제거한다.
        // parse()는 문자->숫자로 변환한다, 정확한 타입명을 명시해주어야한다.

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
