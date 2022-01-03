// slices do not have ownership
// Slices let you reference a contiguous sequence of
// elements in a collection rather than the whole collection.

fn main() {
    let phrase = String::from("Houda is pretty");
    let word = first_word(&phrase);
    // phrase.clear();
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // returns an array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        println!("{}, {}", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

    // let s = String::from("hello world");
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // let slice = &s[0..2];
    // let slice = &s[..2];
    // let len = s.len();
    // let slice = &s[3..len];
    // let slice = &s[3..];
    // let slice = &s[0..len];
    // let slice = &s[..];
}
