#![allow(unused)]

fn modify(s: &mut String) {
  //This does not take ownership of s
  *s += "!"
}

fn main() {
  //Deref
  let mut s = String::from("rust");
  let s1 = &mut s;
  *s1 += "?";
  println!("{s}");

  let mut s = String::from("rust");
  modify(&mut s);
  println!("{s}");

  //Deref coercion
  //Automatically dereferences in some situations
  let x = 1;
  let y = &x;
  let z = &x;
  let w = y + z;
  println!("w = {w}")

}