fn main() {
    //slices can be used with Rusts ranges feature for easy string extraction
    let mut s = String::from("hello world");

    let x = String::from("hello world"); //this is an immutable reference because Rust makes it a &str type because it's a slice

    let hello = &s[0..5];
    let world = &s[6..11];
    let a = &s[..]; // can drop both values to let it know the entire string
    let b = &s[3..]; //can also just drop one or the other if needed to be combined with a certain index

    println!("{} \n{}\n{}\n{}", hello, world, a, b);

    let word = first_word(&s);

    println!("the fist word is {word}");
}

fn first_word(s: &String) -> &str { // get our reference to a string and return the reference
    let bytes = s.as_bytes(); // go through the string by the bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { //check each byte if it's equal to the byte space and return the first word by the index
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_better(s: &str) -> &str { // the subtle difference here is the parameter types, it's better to use &str it allows us to use this function to be used on &String and &str types
    //this tooling gives us flexibility and takes advantage of deref coercions
    let bytes = s.as_bytes(); //

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}