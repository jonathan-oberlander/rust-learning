#[allow(unused_variables)]

// tuples are a way to group data
// tuples can make some abstract coding easier
// be mindful of readability,
// it may be better to create a full data srtucture
// empty tuples are often used

fn main() {
    let some_tuple = (2, 3.4, "a", "b".to_string(), 'z', (1.1, -32.001));
    println!("THe tuple is {:?}", some_tuple);
    println!("My data is {} {}", some_tuple.0, some_tuple.1);

    // acces the nested tuple
    let some_nested_data = (some_tuple.5).1;
    println!("nested tuple at .1 is {}", some_nested_data);

    let some_color = get_some_rgb();
    println!("Green is {}", some_color.1);
    let (red, green, blue) = get_some_rgb();
    println!("Red {}, Green {}, Blue {}", red, green, blue);

    let empty_tuple = ();

    match some_color.2 {
        0..=200 => println!("some color"),
        _ => (),
    }
}

// 256^3 returns more than 16M colors
fn get_some_rgb() -> (u8, u8, u8) {
    // some logic...
    (200, 32, 128)
}
