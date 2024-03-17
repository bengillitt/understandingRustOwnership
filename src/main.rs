// fn main() {


//     // let mut s = String::from("hello");

//     // s.push_str(", world!");

//     // println!("{}", s);


//  // AT this point, rust calls "drop" which automatically deallocated the memory, once it goes out of scope, it gets dropped

// // Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.


//     // let s1 = String::from("hello");
//     // let s2 = s1;

//     // println!("{}, world!", s1); // This doesn't work as if the data is on the heap it creates a "shallow copy" which is when it copys the pointer, length and capacity, s1 becomes invalid

//     // Deep Copying

//     // let s1 = String::from("hello");
//     // let s2 = s1.clone(); // This is so we can create a "deep copy" where we copy the data as well, this means that s1 is still valid

//     // println!("s1 = {s1}, s2 = {s2}");

//     // Ownerships and Functions
    
//     let s = String::from("hello");

//     takes_ownership(s); // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;

//     makes_copy(x); // x would move into the function, but i 32 is Copy, so it's okay to still use x afterward

// }

// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}");
// }


// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// The Slice Type //

// fn first_word(s: &string) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumberate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word will get the value 5

//     s.clear(); // this empties the String, making it equal to ""

//     // word still has the value 5, but there's no more string that we could meaninfully use the value 5 with. word is now totally invalid
// }


// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5]; // We can drop the 0 here as we want to start at 0
//     let world = &s[6..11];
// }



fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]); // word will get the value 5
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);

    // s.clear(); // this empties the String, making it equal to ""

    println!("the first word is: {word}");
}