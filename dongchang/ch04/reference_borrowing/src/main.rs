fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // (1)
    let r2 = &s; // (2)
    // let r3 = &mut s; // (3)
    println!("{} and {}", r1, r2);

    let r4 = &mut s; // (4)
    println!("{}", r4);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}