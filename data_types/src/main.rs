fn main() {

    // NOTE: prefixiing with an underscore removes the problem underlining when a variable is unused.

    //  the 'u32' must be included, as you have to tell the compiler the type of a variable when defining it.
    let _guess: u32 = "42".parse().expect("Not a number!");



    // a scalar type represents a single value

    // different possible integer types
    // signed integers
    let _signed_8_bit: i8 = -117;
    let _signed_16_bit: i16 = -2443;
    let _signed_32_bit: i32 = -534926;
    let _signed_64_bit: i64 = -56743899;
    let _signed_128_bit: i128 = -5924793495;

    // unsigned integers
    let _unsigned_8_bit: i8 = 117;
    let _unsigned_16_bit: i16 = 2443;
    let _unsigned_32_bit: i32 = 534926;
    let _unsigned_64_bit: i64 = 56743899;
    let _unsigned_128_bit: i128 = 5924793495;

    // architecture integers
    // depends on your computer's architecture
    // will be either 32-bit or 64-bit to match your computer's architecture
    let _signed_arch: isize = -53429572;
    let _unsigned_arch: usize = 54923;

    // integer literals
    // integers can be represented with multiple different forms
    // number literals allow a type suffix, like '57u8' to designate the type
    // number literals can also use '_' as a visual separator to make the number easier to read, such as 1_000, which is the same as 1000
    let _decimal = 98_222;
    let _hex = 0xff; // notice the '0x' which shows that this is a hexidecimal-base number
    let _octal = 0o77; // notice the '0o' which shows that this is an octal-base number
    let _binary = 0b1111_0000; // notice the '0b' which shows that this is a binary-base number
    let _byte = b'A'; // u8 only

    // if an integer of type 'u8' is given a value that is larger than an 8-bit unsigned integer, it will overflow, which can result in one of two behaviours
    // the first behaviour is when you're compiling in debug mode, the compiler will 'panic' when the overflow occurs, and it will error and crash.
    // the second behaviour is when you're compiling in release mode, the compiler will perform "two's complement wrapping"
        // this is where the integer value will 'wrap' back around if it exceeds the capacity of the data type, 
        // meaning with a 'u8' the number 256 will become 0, and the number 257 will become 1.

    // floating-point numbers
    // 64-bit is default
    // a 32-bit float is a single-precision float
    // a 64-bit float is a double-precision float
    let _float_64_bit = 2.0;
    let _float_32_bit: f32 = 3.0;

    // numeric operations
    // integer division truncates towards zero to the nearest integer
    
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // booleans
    let _t = true;

    let _f: bool = false; // with explicit type annotation 

    // the character type
    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';



    // compound types can group multiple values into one type

    // tuples
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type
    // tuples have fixed sizes, meaning once they are declared, they cannot grow or shrink in size.
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // the variable tup binds to the entire tuple because a tuple is considered a single compound element
    // to get the individual values out of a tuple, we can use pattern matching to destructure a tuple value
    let (_x, _y, _z) = _tup;
    // x = 500;
    // y = 6.4;
    // z = 1;

    // we can also access a tuple element directly by using a period '.' followed by the index of the value we want to access
    let _five_hundred = _tup.0;
    let _six_point_four = _tup.1;
    let _one = _tup.2;

    // arrays
    // every element must have the same type
    // arrays have a fixed size that cannot be changed once it has been declared
    let _a = [1, 2, 3, 4, 5];

    // arrays are useful when you want your data allocated on the stack rather than the heap
    // you write an array's type using square brackets with the type of each element, a semicolon, and then the number of elements in the array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // you can also initialize an array to contain the same value for each element by specifying the initial value,
    // followed by a semicolon, and then the length of the array in square brackets
    let _a = [3; 5]; // a = [3, 3, 3, 3, 3];

    // an array is a single chunk of memory of a known, fixed size that can be allocated on the stack
    // you can access elements of an array using indexing
    let _first = _a[0];
    let _second = _a[1];

}
