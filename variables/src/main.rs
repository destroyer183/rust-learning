fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // this is erroring because the 'mut' keyword was not included after 'let' where x was defined, which means that x is immutable.
    println!("The value of x is: {x}");
}