fn main() {

/////////////////
//if expresion//
///////////////

    let number = 3;

    /*
     if expressions are called `arms`
     if expresion is not bool --> error
     we'll learn match later to use instade of too many else if 
     values that have the potential to be results from each arm of the if must be the same type
    */
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //inline
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

//////////
//Loops//
////////

    //can exit with ctrl-c 

    /*  
    loop {
        println!("again!");
    }
    */


    //use break
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // <-- ;
    println!("The result is {result}");


    /*
        - break and continue apply to the innermost loop
        - can specify a loop label on loop 
        - it can be used with break or continue to specify that those keywords apply to the labeled loop
        - labels must begin with a single quote
    */

    let mut count = 0;
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

//////////
//while//
////////

    //nesting would be necessary if you use loop, if, else, and break
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    //loop over the elements of a collection
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

////////
//for//
//////

    //loop over the elements of a collection safer --> less bug
    for element in a {
        println!("the value is: {element}");
    }

    //use a Range
    // `rev()` --> reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


}
