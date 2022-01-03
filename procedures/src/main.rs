#[allow(unused_variables)]

fn main() {
    some_function(2.2, 321);
    some_procedure(12.3, -2);

    some_str_procedure("test");

    let string_slice_variable: &str = "a string slice";
    some_str_procedure(string_slice_variable);

    let string_variable: String = String::from("string");
    some_str_procedure(&string_variable); // coercion? just borrow it !
    some_string_procedure(&string_variable); // dont move the ownership to the function !
    some_string_procedure(&string_variable); // its important to reference it in the procedure parameter
}

fn some_string_procedure(param: &String) {
    println!("Param of some_string_procedure is : {}.", param);
}

fn some_str_procedure(param: &str) {
    println!("Param of some_str_procedure is : {}.", param);
}

fn some_procedure(param_a: f32, param_b: i128) {
    println!("I am in  some_procedure {} {}", param_a, param_b);
}

// #[allow(dead_code)]
// #[allow(non_snake_case)]
fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("I am in some_function");

    if param_a < 100. {
        // casting 10 as f32 or 10f32 or 10_f32
        let return_var = 10.1 * param_a + param_b as f32; // should have chosen a better type
        return_var // no semicolon
    } else {
        -1.
    }
}
