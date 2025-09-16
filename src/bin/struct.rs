#![allow(unused)]
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

struct Point3d(f32, f32, f32);

struct Empty;

#[derive(Debug)]
struct Circle {
  center: Point,
  radius: u32,
}

//Struct methods
impl Point {
  //Associated functions (static methods), creates a new instance of the type 
  fn zero() -> Self {
    Self {x: 0.0, y: 0.0}
  }
  //methods
  fn move_to(&mut self, x: f32, y: f32) {
    self.x = x;
    self.y = y;
  }

  fn dist(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }
}

fn main() {
  let p: Point = Point { x: 1.0, y: 2.0 };
  //accessing the struct use dot notation
  println!("point.x: {}", p.x);

  //struct with no keywords
  let p3: Point3d = Point3d(1.0, 2.0, 3.0);
  //accessing the struct use dot notation but now with indexes
  println!("x: {}, y: {}, z: {}", p3.0, p3.1, p3.2);

  //creating a nested struct
  let circle: Circle = Circle { 
    center: Point { x: 2.0, y: 3.0 }, 
    radius: 12 
  };
  //implementing debug for print on individual structs
  println!("{:#?}", circle);

  //shortcut
  let x = 2.0;
  let y = 1.0;
  //since x and y was already defined it will take up the values
  let p: Point = Point { x, y};

  //copy fields
  let p1: Point = Point {x: 1.0, y:1.0};
  let p2: Point = Point { x: 2.0, ..p1};  //similar as spread syntax

  //updating a struct
  let mut p3: Point = Point { x: 0.0, y: 0.0 };
  p3.x += 1.0;
  p3.y += 1.0;
  println!("{:?}", p3);


  //struct methods 
  let mut p:Point = Point::zero();
  p.move_to(2.0, 1.0);

  println!("{:?}", p);

  let d:f32 = p.dist();
  println!("distance: {d}")

}