#[allow(unused_variables)]

fn main() {
    let some_bool = true;
    let some_int = 30;
    let some_int2 = 2;

    if some_bool == true || (some_int > 10 && some_int2 < 10) {
        println!("Hit IF branch");
    } else if some_int == 30 {
        println!("Hit ELSE IF branch");
    } else {
        println!("Hit ELSE branch");
    }

    let var_from_inline = if some_int == 9 {
        200
    } else if some_int == -12 {
        println!("Test");
        400
    } else {
        0
    };

    match some_bool {
        true => {
            println!("Hit true branch");
        }
        false => {
            println!("Hit false branch");
        }
    }

    match some_int2 {
        0 => println!("Hit 0 branch"),
        1..=100 => {
            println!("between 1 and 100 branch");
            println!("more stuff");
        }
        _ => println!("Else branch"),
    }

    let var_from_match = match some_bool {
        true => 10,
        false => 78,
    };

    let var_from_match2 = match some_int {
        0 => 0,
        1 | 2 | 7 => 100,
        _ => 200,
    };
}
