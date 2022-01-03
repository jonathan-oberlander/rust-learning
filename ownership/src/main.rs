#![allow(dead_code)] // to prevent warning during the time of the tutorial
use std::mem;
// The memory is automatically returned once the variable that owns it goes out of scope

fn procedure(x: &Box<i8>) {
    println!("{}", x)
}

#[allow(unused_variables)]
fn main() {
    // types such as integers have a known size at compile time
    // they are stored on the stack.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let stack_i8: i8 = 10;
    let heap_i8: Box<i8> = Box::new(10);
    // let heap_i8_2 = &heap_i8;
    // let heap_i8_3 = heap_i8.clone();
    procedure(&heap_i8);
    println!("{}", heap_i8);

    // x is still valid, nothing moved,
    // a second pointer y to memory exists
    // there’s no difference between deep and shallow copying here
    // Copy types :
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

    // With the String type, in order to support a mutable, growable piece of text,
    // we need to allocate an amount of memory on the heap,
    // unknown at compile time, to hold the contents.
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);
    // when working on the heap, Rust will never
    // automatically create “deep” copies of the data.
    // if s1 is "moved" to s2, you can't access it
    // unless you create a clone;
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    //
    stack_and_heap();
}

// this scope is now over,  s is no longer valid, Rust calls the DROP function.
// that concept is similar to what we do in C++ RAII (Resource Acquisition Is Initialization)
// The memory must be requested from the operating system at runtime.
// We need to return this memory to the operating system when we’re done with the String.

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
}

fn ownership() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
