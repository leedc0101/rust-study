use std::io;
// 원하는 타입이 prelude type에 없다면 use문을 사용해 가져와야 함
// prelude : 자동으로 rust가 가지고 있는 필수 타입
// std 표준 라이브러리
// io 입출력 라이브러리

use rand::Rng;
// Rng trait : 랜덤 숫자 생성기 구현

use std::cmp::Ordering;
// Ordering은 enum이며, Less, Greater, Equal variants를 가짐




fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // thread_rng() : 운영 체제에서 준 시드와 현재 실행 스레드로 만드는 지역변수
    // gen_range() : 범위 내의 랜덤 숫자 생성. start 포함, end 미포함


    // println!("The scret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
        // let : 변수 선언
        // mut : mutable하게 바꿔줌
        // String : 확장 가능한 문자열 타입
        // :: : 앞에 있는 타입만을 위한 연관함수/정적 메소드


        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // & : 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자
        // immutale

        // Result 타입 반환, enums, variants
        // 여기서는 Ok, Err
        // expect는 Result의 메소드
        // Err를 받으면 메시지를 출력하고 프로그램 중지

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // shadow : 이전 guess를 새로운 것으로 덮어씌우도록 해줌
        // String에 있는 trim()메소드는 처음과 끝에 있는 공백문자 제거
        // String에 있는 parse()메소드는 문자열을 숫자로 바꿔줌.
        // 숫자에도 여러 타입이 있기 때문에 u32처럼 타입 명시헤줘야 함
        // parse()는 문자로 변환될 수 없는 글자에서는 에러나기 쉬움
        // 이런 경우 Result타입을 리턴하기 때문에 expect처리해줘야함
        // expect 대신에 match로 바꿈

        println!("You guessed: {}", guess);
        // {} : 변경자, 값이 표시되는 위치

        match guess.cmp(&secret_number) {
         Ordering::Less => println!("Too small!"),
         Ordering::Greater => println!("Too big!"),
         Ordering::Equal => {
            println!("You win!");
            break;
           }
         }

          // cmp : 비교가능한 두 가지 값 비교
             // 비교하고 싶은 값의 레퍼런스(참조자)를 인수로 가짐
             // guess를 secret_number와 비교
             // cmp()의 리턴값에 따른 경우의 수 처리하기 위해 match
             // match는 arm들로 구성됨
             // Arm은 매치할 패턴과, match에 주어진 값이 그 패턴과 일치하면 실행될 코드로 구성됨
    }


}
