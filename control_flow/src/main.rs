

fn main() {
    println!("Hello, world!");
}

fn _branches() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // NOTE: the condition must be a bool, so you can't just do 'if number {}', you would instead do 'if number > 0 {}'
}

fn _if_and_let() {
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

fn _just_loop() {
    loop {
        println!("again!");
    }
}

fn _return_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {result}");
}

fn _loop_labels() {
    let mut count = 0;
    
    // loop labels allow you to specify which loop you want to apply a
    // 'break' or 'continue' to, and they must begin with a single quote, as shown below.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn _while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn _for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }



    // for every number from 1 to 4 in reverse
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}