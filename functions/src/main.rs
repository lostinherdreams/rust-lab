/* 
    - define functions with 'fn'
    + like main!
    - can be defined after or befor main
    - must declare the type of each parameter
    - Rust is an expression-based language
        + statements: instructions that do some action and donot return a value
            let y = 6; --> [but 6 is expresion]
            functions 
            let x = (let y = 6); [can’t assign a let statement to another variable]
        + expressions: evaluate to a resultant value
            5 + 6 --> expresion that evaluates to 11
            calling functions
            macro
            new scope block created with curly brackets
*/

fn main() {
    another_function(4 , 'm');


    let i: i32 = {
        //expresion evaluates to 7, value gets bound to i
        let b = 2i32;
        let c = 5i32;
        b+c // no semicolon!
        /*
            semicolon --> turn expression into statement
            you want b + c to return a value!
        */
    };
    println!("The value of i is: {i}");

    let x = five();
    println!("The value of x is: {x}");
    println!("The value of x in other way is: {}" , five());


    let y = plus_one(5);
    println!("The value of y is: {y}");

}

/*
    parameter --> x [no concrete values]
    argument --> 4 [value that passes by the function]
*/

fn another_function(x:i32 ,  unit_label: char) {
    println!("Another function says: {x}{unit_label}");
}

//return a value
fn five() -> i32 {
    5
    //returns value of the final expression in the block
}

fn plus_one(x: i32) -> i32 {
    x + 1 //no semicolon
}