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

    //int//

    let unsigned_int = 4000;
    let signed_int = -4_000;
    // you can store number 1000 like 1_000 [easier to read]

    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1100;
    let byte = b'A';
    println!("A is {byte} in ASCII"); // ->65


    //float [f32/f64]//

    //default = f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32


    //operation//
    


}
