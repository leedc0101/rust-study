fn main() {
    // error
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // error
    // const y = x * 100;
    // const y = 5 * 100;
    const Y: i32 = 5 * 100_000;
    println!("The value of y is: {}", Y);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", tup.1); // The value of y is: 6.4
	println!("The value of y is: {}", y); // The value of y is: 6.4
}
