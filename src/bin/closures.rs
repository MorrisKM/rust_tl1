#![allow(unused)]


fn main() {
  //closure - anonymous function + capture variables in the environment (scope)
  fn add(x: u32, y: u32) -> u32 {
    x + y
  }
  //write as an anonymous function and can also reassign the variable unlike a function
  let f =|x: u32, y: u32| -> u32 {  x + y };
  //type is inferred
  let f = |x, y| x + y;
  let z = f(1, 2);
  println!("{z}");


  let v = 1;
  let f = |x| x + v;
  let z = f(3);
}