fn main() {
    let test = JohnsStruct { something: 2000 };
    let result = do_this(test);

    let testi32 = 18;
    let result2 = do_this(testi32);
}

trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct JohnsStruct {
    something: i32,
}

impl SomeCustomTrait for JohnsStruct {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        self.something.to_string() + " - " + a + " - " + b
    }
}

impl SomeCustomTrait for i32 {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

// can have multiple traits
#[allow(dead_code)]
fn do_this<T>(some_var: T) -> String
where
    T: SomeCustomTrait + std::fmt::Debug,
{
    println!("{:?}", some_var);
    some_var.blah_blah("first", "second")
}

// here we implemnent traits to the generic types
// its about programming to CAPABILITIES of TYpes via Constraints on Traits
#[allow(dead_code)]
// Constrain using where
fn my_function2<T, U>(arg_a: T, arg_b: T, arg_c: U) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug,
    U: std::fmt::Debug,
{
    println!("{:?}", arg_a);
    println!("{:?}", arg_c);
    arg_a - arg_b
}

// Constrain
fn my_function<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
    arg_a: T,
    arg_b: T,
) -> T {
    println!("{:?}", arg_a);
    arg_a - arg_b
}

// specify one and only one trait per variable
#[allow(dead_code)]
fn do_this_2(some_var: &dyn SomeCustomTrait) -> String {
    println!("{:?}", some_var); // compile error
    some_var.blah_blah("first", "second")
}

enum SomeEnum<T> {
    OptionA(T),
    OptionB(T),
    OptionC,
}

fn main() {
    let some_data = SomeEnum::OptionA(32.3);
    match some_data {
        SomeEnum::OptionA(a) => println!("OptionA {}", a),
        SomeEnum::OptionB(b) => println!("OptionB {}", b),
        SomeEnum::OptionC => println!("Boring option c :/"),
    }
    let some_data_2 = SomeEnum::OptionB("c");
    let some_data_3 = SomeEnum::OptionA(vec![1, 2, 3]);
}

struct Point<T, U> {
    x: T,
    y: U,
}

struct JohnsData<T> {
    x: i32,
    y: T,
    z: T,
}

fn main() {
    let a = Point {
        x: 3.23_f64,
        y: 8.89_f32,
    };
    println!("x={} y={}", a.x, a.y);
    let b = Point { x: 2_u16, y: -1 };
    println!("x={} y={}", b.x, b.y);
    let c = JohnsData {
        x: 2,
        y: -1.12,
        z: 4.,
    };
    println!("x={} y={} z={}", c.x, c.y, c.z)
}
