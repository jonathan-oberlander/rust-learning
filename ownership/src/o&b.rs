fn main() {
  let stack_i8: i8 = 10
  let heap_i8: Box<i8> = 10
  let heap_i8_2 = heap_i8
  println!("{}, {}", heap_i8_2, heap_i8)
}