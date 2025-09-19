#![allow(unused)]

use std::{fmt::Debug};

//to print a generic type impliment the debug trait
fn f<T : Debug>(t: T) {
  println!("{:?}", t)
}

//trait bound - specifies constraints on a generic type

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for u32 {}
impl A for i32 {}

fn c<T: A>(x: T) {}

fn m<T: A + B>(x: T) {}

fn w<T: A + B, U: B+ C>(x: T, y: U) {}
//can also be written as
fn y<T, U> (x: T, y: U)
where 
  T: A + B,
  U: B + C
{}

//Difference between impl trait syntax and trait bounds
//x and y can be different types
fn k(x: impl A, y:impl A) {}
// x and y must be of same type
fn g<T: A>(x: T, y: T) {}
//unless
fn v<T: A, U: A>(x: T, y: U) {}

fn main() {
  let u:u32 = 1;
  let i:i32 = -1;
  let f:f32 = 1.0;

  c(u);
  c(i);

  m(u);

  w(u, u);

}