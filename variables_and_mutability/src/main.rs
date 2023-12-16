use std::io::stdin;


fn get_user_input() -> u32{
    let mut num = String :: new();
    stdin().read_line(&mut num).expect("msg");
    let num:u32 = num.trim().parse().expect("Not a number!");
    return num;
}

fn main() {

///////////////
//Mutability//
/////////////

   let mut x = 5;
    println!("Value of x is : {x}");
    x = 6;
    /*with out `mut` cannot assign twice to immutable variable `x`;
    variables are immutable by default*/
    println!("Value of x is : {x}");

//////////////
//Constants//
////////////

    /*constants are values that are not allowed to change[kinda like immutibale variables]
    -  cant use mut with constants.
    -  declare constants using the const
    -  type of the value must be specified <next chap>
    -  can be declared in any scope, including the global scope
    -  determined at compile time
    -  constants are valid for the entire time a program runs
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand 
    println!("THREE_HOURS_IN_SECONDS is :{THREE_HOURS_IN_SECONDS}");
    /* const USER_INPUT_VALUE:u32 = get_user_input();
    -  cannot call non-const fn `get_user_input` in constant*/

//////////////
//Shadowing//
////////////

    /*you can declare a new variable with the same name as a previous variable
    -> the first variable is shadowed by the second
    -> the second variable is what the compiler will see in the scope
    */
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("number of spaces are: {spaces}");

    // we can change the type of the value using shadowing but not `mut`
    /*
      -NOT VALID- 
      let mut spaces = "   ";
      spaces = spaces.len();
      println!("number of spaces is: {spaces}"); 
    */





}
