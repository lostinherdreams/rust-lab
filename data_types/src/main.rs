/*
  Rust is a statically typed language
  -> must know types of variables at compile time
  - compiler usually understand what type we want to use 
     based on the value and how we use it
  - In cases when many types are possible
     such as when we converted a String to a numeric type using parse
  -> we must add a type annotation
 */
fn main() {

    let guess: u32 = "42".parse().expect("Not a number!");
    // with out u32 -> error: type annotations needed

/////////////////
//Scalar Types//
///////////////

    /*
      - scalar type represents a single value
      + four pimary scalar types:
        - integers [isize/usize]
        - floating-point numbers
        - Booleans
        - characters
    */

    //***int***//

      // i32 --> [2^(-31) , 2^31 - 1]

      let unsigned_int = 4000;
      let signed_int = -4_000;
      // you can store number 1000 like 1_000 [easier to read]

      let hex = 0xff;
      let oct = 0o77;
      let bin = 0b1100;
      let byte = b'A';
      println!("A is {byte} in ASCII"); // ->65

      // overflow -> panic in debug mode / 2th complement in release mode


    //***float [f32/f64]***//

      //default = f64
      let x = 2.0; // f64
      let y: f32 = 3.0; // f32


    //***operation***//

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
  
    //***boolean type***//

      //one byte in size
      let t = true;

      let f: bool = false; // with explicit type annotation

    //***character type***//

      /*
        -four bytes in size
        -represents a Unicode Scalar Value
        -range from U+0000 to U+D7FF and U+E000 to U+10FFFF
      */
      let c = 'z';
      let z: char = 'ℤ'; // with explicit type annotation
      let heart_eyed_cat = '😻';


////////////////////
//Coumpound Types//
//////////////////

    /* 
      - can group multiple values into one type 
        + tuples
        + arrays
    */

    //***tupple type ***//

      /*
        - grouping together values with diffrent types 
        - fixed length
        - unit :empty tupple
          +value and type --> ()
      */
      let tup: (i32, f64, u8) = (500, 6.4, 1);

      //accesss
      let tup = (500, 6.4, 1);

      let (x, y, z) = tup; //destructuring
      println!("The value of y is: {y}");

      let x = tup.0;
      println!("The value of tup[0] is: {x}");


    //***array type***//

      /*
        - every element of an array must have the same type
        - fixed length
        - allocate data on the stack rather than the heap 
        - for when you know the number of elements dont need to change
      */
      let a = [1, 2, 3, 4, 5];

      let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

      let a = [3; 5]; // --> [3,3,3,3,3]

      let first = a[0]; //index out of bound --> panick


}
