#![allow(unused)]

fn main() {
  let x: Option<i32> = Some(2);
  let z: Option<i32> = Some(5);
  //when using unwrap() the code should return Some() or panic without a message
  let v = z.unwrap();
  println!("{v}");
  //with expect the message passed is like the None in a pattern match 
  let y = x.expect("there is no value");
  println!("{y}");

  let a: i32 = 3;
  let b: i32 = 3;
  let a: Result<i32, String> = Ok(a/b);
  let d = a.unwrap();
  println!("{d}")
}