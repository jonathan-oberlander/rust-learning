// structs represent complex data types that you define
// structs are like "objects" - not OOP tho
// Rust doesnt have inheritance
// Rust doesnt have methods
// Rust has Traits - Similar to Polymorphism in OOP
// Derived traits can be done with macros

// to import a file call the mod keyword
// its also handy to add use
mod random_info;
use random_info::*;

// lets add a struct
// the Debug trait annotates the struct that use it
// its important to annotae to the all other composed Traits
#[allow(dead_code)]
#[derive(Debug)]
struct JohnsData {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
    random: RandomInfo, // composition of a struct within a struct, no inheritance
}

// it is possible to add an impl to our struct from anywhere !!!
impl RandomInfo {
    pub fn is_larger(&self, compare_to: i32) -> bool {
        self.some_int < compare_to
    }
}

// Rust has default traits such as Default, Clone etx...
// Here the Defualt trait allows us to create a default JohnsData
impl Default for JohnsData {
    fn default() -> Self {
        Self {
            some_bool: true,
            some_float: 33.6,
            some_int: -14,
            random: RandomInfo::new(false),
        }
    }
}

// SomeTrait trait was decalred in radom_info.rs file
impl SomeTrait for JohnsData {
    fn is_valid(&self) -> bool {
        self.some_bool
    }
}

#[allow(unused_variables)]
fn main() {
    // we can create a struct like so
    let mut random_info_var = RandomInfo {
        call_count: 0,
        some_bool: false,
        some_int: 20000,
    };

    // lets use our new method that returns an instance of the RandomInfo Struct
    // we use the :: because the method returns a new instance of itself (Self)
    let another = RandomInfo::new(false);
    let mut and_another = RandomInfo::new(true);

    // if the default trait was implemented for the struct
    // we can create a struct like that as well :)
    let mut johns_var = JohnsData::default();

    // lets mutate some values
    johns_var.some_int = 3000;
    and_another.some_bool = false;

    // the .. notation equivalent to spread operator in JS
    // in that case it will move johns_var to johns_var_2
    let johns_var_2 = JohnsData {
        some_float: -33.458,
        ..johns_var
    };

    // lets call a method implemented on the struct,
    // since we are returning a value, the function call is a dot operator
    let is_smaller = random_info_var.is_smaller(32);
    let is_larger = random_info_var.is_larger(2);
    random_info_var.add_to_count(2);
    random_info_var.add_to_count(12);
    random_info_var.add_to_count(7);
    println!("{}", random_info_var.call_count);

    let is_valid = random_info_var.is_valid();
    let is_john_valid = johns_var_2.is_valid();

    // since both JOhnsData and RandomInfo implement our trait
    // we use that function on both
    // the same data type is used
    print_if_is_valid(&random_info_var);
    print_if_is_valid(&johns_var_2);

    // that syntax {:?} allows us to Debug trait,
    // Debug is a trait we can import and use as a macro
    // rather than implementing it manually ourselves :D
    println!("{:?} {:?}", another, and_another);
    println!("{:?}", johns_var_2);
}

fn print_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Yay!! Its Valid :)")
    }
}
