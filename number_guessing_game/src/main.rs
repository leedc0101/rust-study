use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut count = 0;

    loop {
        count += 1;
        println!("숫자를 맞춰보세요! ({}번째 시도)", count);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("숫자를 입력하세요!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다! \n"),
            Ordering::Greater => println!("입력한 숫자가 큽니다! \n"),
            Ordering::Equal => {
                println!("정답!");
                break;
            },
        }
    }
}