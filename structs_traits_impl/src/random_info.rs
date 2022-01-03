// because random info will be used in another file
// we need to tell which field is public in the struct
#[derive(Debug)] // the trait anotates the struct that use it
pub struct RandomInfo {
  pub call_count: i32,
  pub some_bool: bool,
  pub some_int: i32,
}

// associated functions are implementation
// they use the impl followed by the type we are implementing for
// inside are the funcitons and methods
// the convention of an implementation block is to return Self
// in our case Self is the RandomInfo Type that we are implementing for

impl RandomInfo {
  // here creating a new function allows us a way to return the struct and initialize its parameters
  // that is a bit like a constructor with a paramaeter and two fixed values
  // it creates a new struct
  pub fn new(param_a: bool) -> Self {
    Self {
      call_count: 0,
      some_bool: param_a,
      some_int: 8,
    }
  }

  // this is how you can modify the data of your struct internally
  // self is similar to this keyword in JS
  // lower case self means it represents actual data of the implementation
  // and not the struct itself

  // here we reference the struct
  pub fn is_smaller(&self, compare_to: i32) -> bool {
    self.some_int < compare_to
  }

  // here we reference the struct and mutate one of its values
  pub fn add_to_count(&mut self, amount: i32) {
    self.call_count += amount;
  }
}

// polymorphism is possible with traits,
// traits allow us to modify structs but
// they can be used on multiple structs !!!!
pub trait SomeTrait {
  fn is_valid(&self) -> bool;
  // fn is_the_better_one(&self, some_other_dude: &Self) -> Self;
}

// this is how to implement a trait on a struct
// fns in structs don't need to be public
impl SomeTrait for RandomInfo {
  fn is_valid(&self) -> bool {
    self.some_bool
  }
}

// uni structs
#[allow(dead_code)]
struct AStructWIthNoField {}

// generic in structs
#[allow(dead_code)]
struct Pair<T> {
  x: T,
  y: T,
}

// tuple structs have a type name whereas tuples doesnt
#[allow(dead_code)]
struct Color(u8, u8, u8);
