fn main() {
    another_function(5);

    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn _expressions() {

    // this is a statement
    let _y = 6;

    // because statements do not return values, you cannot assign a 'let' statement to another variable
    // the following code will error at compile-time if it is uncommented
    // let _x = (let _y = 6);

    // since expressions have a return value, the following code will function properly
    let _y = {
        let x = 3;
        x + 1
    };
    // the reason the 'x + 1' doesn't have a semicolon is because that would make the computer consider it as a statement, which won't have a return value.

}

// functions can return values to the code that calls them. 
// we don’t name return values, but we must declare their type after an arrow '->'
fn five() -> i32 {
    // You can return early from a function by using the return keyword and specifying a value, 
    // but most functions return the last expression implicitly
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // this code will cause an error, as the semicolon makes the computer consider this line as a statement, which don't return values.
}