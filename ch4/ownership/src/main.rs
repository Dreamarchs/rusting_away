fn main() {
    let mut s = String::from("hello"); //the double colon operator is for namespacing the from function in  this instance called method syntax
    s.push_str(", world!"); // this appends a sting to s as a string literal.
    //literals cannot be mutated
    /*
    NOTE: Rust frees variable memory depending on if the variable is out of scope or not automatically.
    */
    println!("{s}");

    let x = 5;
    let y = x;
    //The above y variable automatically creates a copy of the value that x has since it's a simple i32 number.

    let s1 = String::from("hello");
    let s2 = s1;
    //The Above variables share the point in memory on the heap, this is the more common approach in rust for variables of more complexity
    //the above also means s1 is no longer valid to avoid double freeing

    let s3 = s2.clone();
    //the above create an real copy completely free of any binding of s2 it would have without .clone()
}
