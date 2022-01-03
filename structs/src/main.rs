#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // references allow you to keep ownership of the data in main
    area2(&rect1);

    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let my_email = String::from("my@my.com");
//     let my_user = String::from("john0");
//     let mut user1 = build_user(my_email, my_user);
//     user1.email = String::from("anotheremail@example.com");

//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         ..user1
//     };
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// to reference in struct you need lifetime parameters
// struct Us {
//     username: &str,
//     email: &str,
// }
