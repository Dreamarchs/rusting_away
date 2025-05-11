fn main() {
    let mut s = String::from("hello"); //the double colon operator is for namespacing the from function in  this instance called method syntax
    s.push_str(", world!"); // this appends a sting to s as a string literal.
    //literals cannot be mutated
    /*
    NOTE: Rust frees variable memory depending on if the variable is out of scope or not automatically.
    */
    println!("{s}");

    takes_ownership(s); //since we pass s to the function it's moved, and the function now owns it

    let x = 5;
    let y = x;
    //The above y variable automatically creates a copy of the value that x has since it's a simple i32 number.

    makes_copy(y);
    println!("{x} and {y}"); //we can still call x + y because even though we passed them to a function, simple structs that are only on the stack
    // are easily copied so Rust handles them that way. integers, floats, bools and tuples that only contain the simple types listed can also implement the Copy trait

    let s1 = String::from("hello");
    let s2 = s1;
    //The Above variables share the point in memory on the heap, this is the more common approach in rust for variables of more complexity
    //the above also means s1 is no longer valid to avoid double freeing

    let s3 = s2.clone();
    //the above create an real copy completely free of any binding of s2 it would have without .clone()
}


fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn  makes_copy(some_integer: i32) {
    println!("{some_integer}");
}