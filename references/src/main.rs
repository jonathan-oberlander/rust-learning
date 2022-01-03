// At any given time, you can have either one mutable reference
// or any number of immutable references.
// References must always be valid.

fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    change(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    mult_refs();
    dont_mutate_multi_refs();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope.
  // Because it does not have ownership of what
  // it refers to, nothing happens.

// we need to anotate the reference as mutable to change it
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// you can have only one mutable reference to a particular
// piece of data in a particular scope
// Rust prevents data races to happen
// Only one reference => only one pointer
// Rust wont compile code with data races
// howere we can use curlys to create a new scope, allowing
// for multiple mutable references, just not
// simultaneous ones !!!

fn mult_refs() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    // r1 goes out of scope here,
    // we can make a new reference, no problems.
    let r2 = &mut s;
}

fn dont_mutate_multi_refs() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // ERROR !!
    println!("{}, {}, and {}", r1, r2, r3);
    // We also cannot have a mutable
    // reference to an immutable one.
    // However, multiple immutable references are okay.
}

fn refs_scope() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn dangle() -> &String {
    // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
}
// Here, s goes out of scope, and is dropped.
// Its memory goes away.
// The solution here is to return the String directly:

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
