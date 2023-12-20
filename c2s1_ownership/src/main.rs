/*
   - Rust check at compile-time whether a variable is defined before being used 
   - Variables Live in the Stack
   - Boxes Live in the Heap
   - stack memmory mange like other langueges
   - we dont have `free()` for heap
   - Boxes ownership transfers! 
*/

fn main() {

    // a is a variable in stack storing a pointer to an array in heap
    let a = Box::new([0; 1_000_000]); 
    let b = a; 
    
    let a = Box::new([0; 1_000_000]);
    let b = a;

    //cant: `println!("{a}");` a is not owener of box anymore!

    
    //Structures like Vec, String, and HashMap use box
   
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
    //Variables Cannot Be Used After Being Moved!

    //Cloning Avoids Moves
    let first = String::from("Ferris");
    let first_clone = first.clone(); // clone --> to strings in heap
    //"deep" copied the string data into a new heap allocation
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");

}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}