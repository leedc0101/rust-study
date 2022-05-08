#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(s) => {
            println!("State quarter from {:?}!", s);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let num = value_in_cents(coin);
    println!("{}", num);
}