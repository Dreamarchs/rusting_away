fn main() {
    let s1 = String::from("hello");
    let(s2, len) = calculate_length(s1);
    let len2 = calculate_length(&s1);

    println!("The Length of '{s2}' is {len}.");

    println!("The Length of '{s1}' is {len2}.");

    //mutability
    // references without the mut keyword cannot be referenced and changed
    //for this to work the value AND reference both have to have the mut keyword
    //NOTE: cannot have 2 mutable references to a value at the same time if you have one it must be cleaned up or dropped before creating another
    //Also cannot have static references then a mut one but can have multiple static references
    //Order of the code is very important to Rust when it comes to ownership and using variables


    let mut a = String::from("hello");

    change(&mut a);
}

//without using a reference
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // rust can return multiple values inside a tuple to another variable of type tuple
    //we had to return the string because we moved ownership, but there is a way to avoid this
    //it's known as referencing in Rust that is like a pointer,
    //it has an address to follow and that contains data owned by a different value
    //but in Rust the data is guaranteed to point to a valid value of a particular type for the life of the Ref
}

//using references
fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

//NOTE: Opposite of references (&) is a dereference (*)

//mutability
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}