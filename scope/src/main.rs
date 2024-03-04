
fn main() {
    println!("Hello, world!");
}

fn _scope() {
        {                       // s is not valid here, it’s not yet declared
        let _s = "hello"; // s is valid from this point forwardK
                                
        // do stuff with s
    }                           // this scope is now over, and s is no longer valid
 
}

fn _strings() {
    let _string = "hello"; // this is a 'string literal' which is immutable, and is stored on the stack. its size cannot change.

    // using String::from(), you can create a 'String' from a 'string literal'
    let mut s = String::from("hello"); // this is a 'String' which is stored on the heap, and is able to store an unknown amount of text.

    s.push_str(", world!"); // 'push_str()' appends a 'string literal' to a 'String.'

    println!("{s}"); // this will print 'hello, world!'



}