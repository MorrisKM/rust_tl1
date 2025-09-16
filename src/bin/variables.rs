#[allow(unused)]

fn main() {
  let x: i32 = -123;
  //variables are immutable by default
  //x += 1 this will not work
  let mut y: i32 = 123;
  y += 1;

  const NUM: u32 = 1;

  //redeclaring a variable
  let x: i32 = -1;
  let x: bool = true;

}