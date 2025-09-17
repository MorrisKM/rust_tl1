#![allow(unused)]

fn main() {
  let x: i32 = 4;
  //if statement
  if x % 2 == 0 {
    println!("{x} is an even number")
  }else{
    println!("{x} is an odd number")
  }

  //returning a value from an if conditional statement
  let z: i32 = if x > 0 {
    1
  }else if x < 0 {
    -1
  }else{
    0
  };

  println!("{z}");
}