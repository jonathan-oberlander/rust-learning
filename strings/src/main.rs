#[allow(unused_variables)]

fn main() {
    // strings are groupings of types u8
    // that represent text
    // diff is how its sorted in memory

    // Strings are stored on the Heap
    // Strings are mutable

    // &str (string slice)
    // immutable
    // often allocated on the stack
    // sometimes can be a heap reference
    // sometimes embeded in the code itself

    // you can translate betweem the two types

    // Strings are great for mutatin and hold data longer than the stack is able to
    // &str (string slice) are great for runtime speed // since they are stored on the stack

    // a string slice
    let example_str: &str = "Howdy";

    // cast &str to String, using .to_string() method
    // a more maleable form of a string, allocated to the heap.
    let string_from_str: String = example_str.to_string();

    // hardcoded String, also needs .to_string().
    // they are somtime called "string literals"
    // they are &str (string slice) that are held in progrma binary or "static memory"
    // They have all the properties of a string slice
    let string_from_str2: String = "Some Hardcoded string".to_string();
    let string_from_hardcoded = String::from("some hardcoded string");
    let example_string: String = String::from("Partners");

    // you can also cast &str to String using from and passing the string slice
    let string_from_str_var = String::from(example_str);

    // this is a borrow of a string slice reference (stored on the stack)
    let str_from_string: &str = &example_string;

    // here is how to append a String, we use concat, the new String is stored on the Heap
    let combine_string_literals = ["first, ", "second"].concat();
    // you can also use the format! macro to concat them and return a new String
    let combine_with_format_macro = format!("{}, {}", "first", "second");
    // that works also
    let string_plus_str = example_string + "something";
    // append a &str to a String works ONLY if if the String is first
    // that borrow is a cheap trick though
    let string_plus_str2: String = "something".to_string() + &example_str;

    // here is a nice way to append strings to String
    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("sime hardcoded literal");

    // push a character to a String,
    // Chars are defined in single quotes !!!
    mut_string.push('z');

    // here
    let a = String::from("a");
    let z = String::from("z");
    // that works with a borrow, but "moves" a to combined,
    let combined = a + &z;
    // println!("{}", a); // error => value borrowed after move

    // you can get substrings with a borrow
    let example_string_2: String = String::from("Howdy");
    let str_from_subsstring: &str = &example_string_2[3..5]; // [..4]

    // it is possible to Reassign a string slice
    let example_str_2: &str = "Partners";
    let str_from_subsstring: &str = &example_str_2[..=2]; // [4..]

    // how to get a character at a specific index
    let char_by_index = &example_str_2.chars().nth(1);
    // error handling with chars or strings with a match !!!
    match char_by_index {
        Some(c) => println!("found a char {}", c),
        None => {}
    }
    // if let is very handy in that case
    if let Some(c) = example_str.chars().nth(3) {
        println!("foud a char {}", c)
    }
}
