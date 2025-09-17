#![allow(unused)]

fn main(){
  let x: Option<i32> = Some(123);
  if let Some(v) = x {
    println!("Some: {v}"); //will print if the value of x is Some(val)
  }

  let Some(v) = x else {
    panic!("The value does not exist") //program will panic if the value of x is none
  };
}