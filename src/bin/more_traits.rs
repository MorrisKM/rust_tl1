#![allow(unused)]

trait Color {
  fn get(&self) -> String;
}

trait Rectangle {
  fn get(&self) -> (i32, i32, u32, u32);
}

struct Square {
  color: String,
  top: i32,
  left: i32,
  size: u32,
}

impl Color for Square{
  fn get(&self) -> String {
      self.color.clone()
  }
}

impl Rectangle for Square{
  fn get(&self) -> (i32, i32, u32, u32) {
      (self.top, self.left, self.size, self.size)
  }
}

fn main() {
  let square: Square = Square { color: "red".to_string(), top: 0, left: 0, size: 10 };
  //since bot square and rectangle have get functions we use the fully qualified syntax
  let color: String = Color::get(&square);
  let (x, y,width, length) = Rectangle::get(&square);

  println!("color is {color}");
  println!("x: {x}, y: {y}, width: {width}, length: {length}")
}