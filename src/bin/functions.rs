#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
  x + y
}

//Diverge funcitons, do not return a value
fn crash() -> ! {
  panic!("crash")
}
fn main() {
  let y: u32 = 1;
  let x: u32 = 2;
  let z = add(x, y);
}