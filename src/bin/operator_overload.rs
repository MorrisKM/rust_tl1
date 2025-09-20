#![allow(unused)]
use std::ops::Add;

//operator overload
#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

//the anatomy of the add
/* 
pub trait Add<Rhs = self> {
  type Output;

  const fn add(self, rhs: Rhs) -> Self::Output;
}
*/

impl<T> Add for Point<T> where T: Add<Output = T>  {
  type Output = Point<T> ;
  fn add(self, rhs: Point<T> ) -> Self::Output{
    Point{
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

fn main() {
  let p0: Point<i32> = Point { x: 1, y: 2 };
  let p1: Point<i32> = Point { x: 3, y: 4 };

  //instance of the add
  let p2: Point<i32> = p0 + p1;
  println!("{:?}", p2);
}