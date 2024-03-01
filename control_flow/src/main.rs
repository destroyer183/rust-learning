

fn main() {
    branches();

    if_and_let();
}

fn branches() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // NOTE: the condition must be a bool, so you can't just do 'if number {}', you would instead do 'if number > 0 {}'
}

fn if_and_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn _if_and_let_bad() {
    let _condition = true;
    // let number = if condition { 5 } else { "six" }; 
    // the above line will break, as there is no definitive type for the if statement,
    // as it could either be an integer or a string depending on whether or not 'condition' is true.

}