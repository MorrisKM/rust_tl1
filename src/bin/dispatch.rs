#![allow(unused)]

//static and dynamic dispatch

//Static dispatch
// - function to call is known at compile time 
// - monomorphization (code size can be larger)
// - no run time cost (no vtable lookup)

//Dynamic dispatch
// - function to call is known at run time not compile table
// - vtable lookup (code size is smaller)
// - run time overhead (vtable lookup)

use std::sync::Arc;

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

trait F {
  fn f(&self);
}

impl F for A{
  fn f(&self) {
      println!("{:?}", self)
  }
}

impl F for B{
  fn f(&self) {
      println!("{:?}", self)
  }
}

fn static_dispatch<T: F>(t: &T) {
  t.f()
}

fn dynamic_dispatch_box(t: Box<dyn F>) {
  t.f();
}

fn dynamic_dispatch(t: &dyn F) {
  t.f();
}

fn main() {
  let obj = A;
  static_dispatch(&obj);

  let obj = B;
  static_dispatch(&obj);

  let input = "A";
  let obj: Box<dyn F> = match input {
    "A" => Box::new(A),
    "B" => Box::new(B),
    _ => panic!()
  };
  //note that using the Box transfers ownership the the dynamic_dispatch_box function and cannot be used after the function call
  dynamic_dispatch_box(obj);

  //also since we just want to know the size at compile time
  let input = "B";
  let obj2: &dyn F = match input {
    "A" => &A,
    "B" => &B,
    _ => panic!()
  };
  //does not transfer ownership since it passes a reference to the struct
  dynamic_dispatch(obj2);
}