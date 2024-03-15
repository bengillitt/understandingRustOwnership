fn main() {


    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{}", s);


 // AT this point, rust calls "drop" which automatically deallocated the memory, once it goes out of scope, it gets dropped

// Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.


    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1); // This doesn't work as if the data is on the heap it creates a "shallow copy" which is when it copys the pointer, length and capacity, s1 becomes invalid

    // Deep Copying

    // let s1 = String::from("hello");
    // let s2 = s1.clone(); // This is so we can create a "deep copy" where we copy the data as well, this means that s1 is still valid

    // println!("s1 = {s1}, s2 = {s2}");

    // Ownerships and Functions
    
    let s = String::from("hello");

    takes_ownership(s); // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;

    makes_copy(x); // x would move into the function, but i 32 is Copy, so it's okay to still use x afterward

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}