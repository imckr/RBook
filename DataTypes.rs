let guess: u32 = "42".parse().expect("Not a number!");

// 8bit --> i8 --> u8
// 16bit --> i16 --> u16
// 32bit --> i32 --> u32
// 64bit --> i64 --> u64
// 128bit --> i128 --> u128
// arch --> isize --> usize

// Here’s an example that shows floating-point numbers in action:

fn floatExample() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

// Numeric Operations

fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // Tuple Type

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");


    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Array Type

    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let c = [3; 5]; // let c = [3, 3, 3, 3, 3];


}