fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
} // AT this point, rust calls "drop" which automatically deallocated the memory, once it goes out of scope, it gets dropped

// Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.
