#![allow(unused)]

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let x = "hello".to_string();
  {
    let y = "rust".to_string();
    let z = longest_str(&x, &y);
    println!("z = {z}");
  }
}