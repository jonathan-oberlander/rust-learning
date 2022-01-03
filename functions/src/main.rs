fn main() {
    match_statement();
    // break_loop();
    // let num: u32 = 6;
    // f_to_c(98);
    // let fac = factorial(num);
    // let fibo = fibonacci(num);
    // println!("number {}, factorial is {}, fibo is {}", num, fac, fibo);
    // lyrics();
}

fn match_statement() {
    let country_code = 24; // 1 999
    let country = match country_code {
        7 => "RU",
        33 => "FR",
        44 => "UK",
        46 => "SW",
        _ => "unknown",
    };
    println!("the country code {} is {}", country_code, country)
}

// fn f_to_c(temp: i16) -> i16 {
//     let convert = (temp - 32) * 5 / 9;
//     println!("{}°F is {}°C", temp, convert);
//     convert
// }

// fn factorial(n: u32) -> u32 {
//     if n == 0 {
//         1
//     } else {
//         n * factorial(n - 1)
//     }
// }

// fn fibonacci(n: u32) -> u32 {
//     match n {
//         0 => 1,
//         1 => 1,
//         _ => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }

// fn lyrics() {
//     let days = [
//         "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
//         "tenth", "eleventh", "twelfth",
//     ];

//     let objects = [
//         "a partridge in a pear tree ",
//         "two turtle doves and ",
//         "three French hens, ",
//         "four calling birds, ",
//         "five gold rings, ",
//         "Six geese a laying, ",
//         "seven swans a swimming, ",
//         "eight maids a milking, ",
//         "nine ladies dancing, ",
//         "ten lords a leaping, ",
//         "eleven pipers piping, ",
//         "twelve drummers drumming, ",
//     ];
//     let mut phrase: String = "".to_owned();

//     for number in 0..12 {
//         phrase.insert_str(0, objects[number]);
//         println!(
//             "On the {} day of Christmas my true love gave to me",
//             days[number]
//         );
//         println!("{}\n", phrase);
//     }
// }

// fn main() {
//     println!("Hello, world!");
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     //  If we add a semicolon to the end of an expression,
//     //  it becomes a statement, it will not return a value.
//     another_function(45, y);
//     println!("In Main, y is: {}", y);
//     if_condition(247);
//     if_in_let();
//     break_loop();
// }

// fn another_function(x: i32, y: i32) {
//     println!("In O_func, x is: {}", x);
//     println!("In O_func, y is: {}", y);
//     let z = five();
//     println!("In O_func, z is: {}", z);
//     let six = plus_one(5);
//     println!("In O_func, six is: {}", six);
// }

// fn five() -> i32 {
//     // functions return the last expression implicitly.
//     5
// }

// fn plus_one(x: i32) -> i32 {
//     // if we add a semicolon, the expression becomes a statement.
//     // statements don’t evaluate to a value
//     x + 1
// }

// fn if_condition(number: i32) {
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }

//     if number != 0 {
//         println!("number was something other than zero");
//     }

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn if_in_let() {
//     // similar to ternary operator
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {}", number);
// }

// fn break_loop() {
//     let mut counter = 0;

//     // stored the loop expretion in the variable
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);

//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     // while loop, unsafe.
//     while index < 5 {
//         println!("the value is: {}", a[index]);
//         index += 1;
//     }

//     // forEach iteration
//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }

//     // for loop
//     for number in (1..4).rev() {
//         println!("{}!", number);
//     }
//     println!("LIFTOFF!!!");

//     for (position, y) in (67..88).enumerate() {
//         println!("{}: {}", position, y)
//     }
// }
