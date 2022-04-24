//bring input,output library
//std::io isn't prelude type, so have to use with a use statement
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("Secret number is: {}",secret_number);
    
    loop {
        println!("Please input your guess.");
        //variable init by let(immutable) and mut(mutable)
        //String type, new?? > new instance of String >> associated function
    
        let mut guess = String::new();
        //call stdin func from io, can be replaced as std::io:;stdin if there is not "use std::io"
        io::stdin()
            //& means argument is a reference?? what is reference
            //references are immutable so write mut for mutable guess
            .read_line(&mut guess)
            //read_line methods return value Result(enum, Ok or Err)
            //if err print failed~ should use expect with readline or will cause err
            .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        //{} placeholder can be multiple
        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too samll!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Win");
                break;
            }
        }
    }
}
