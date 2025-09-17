#![allow(unused)]

fn main() {
  let a: i32 = 42;
  let b: i32 = 5;

  //operators -,+,*,/
  let c = a * b;
  let c: f32 = (a as f32) / (b as f32);
  let c: i32 = a + b;
  let c: i32 = a - b;

  //% mod
  println!("{}", a % b);
  println!("{}", b % a);

  //literals 
  let a = 1i32;
  let b = 2u64;
  let c = 3f64;
  let d: f64 = 3e4; //30000
  //to add readability while working with big numbers add underscores after every 3 figures
  println!("{}", d);

  //Booleans
  let a: bool = true && false; //have a value false
  let b: bool = true || true; //have a value true
  let c: bool = !true;
  println!("{}", a)

}