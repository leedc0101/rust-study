fn main() {
    let s = String::from("hello");
    takes_ownership(s); // Move
    // 더 이상 s에 접근할 수 없음

    let x = 5;
    makes_copy(x); // Copy
    // x에 계속 접근 가능
    
    let s2 = String::from("hi");
    let s3 = take_and_give_back(s2);
    
  	// println!("{}", s); // Error
    println!("{}", s3); // hi
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn take_and_give_back(some_string: String) -> String {
    println!("{}", some_string);
	some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}