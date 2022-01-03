fn main() {
    // const MAX_POINTS: u32 = 100_000;

    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("{}", spaces);

    // let x = 2.0; // f64

    // let y: f32 = 3.0; // f32

    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;

    // // remainder
    // let remainder = 43 % 5;

    // let t = true;

    // let f: bool = false; // with explicit type annotation

    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // destructuring through pattern matching

    println!("The value of x y z are: {}, {}, {}", x, y, z);

    let my_tup: (i32, f32, u8) = (1200, 3.44, 7);
    let milledeuxcent = my_tup.0;
    println!("{} {} {}", milledeuxcent, my_tup.1, my_tup.2);

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i8; 5] = [1, 2, -3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    let _b = [3; 5]; // [3, 3, 3, 3, 3]
}
