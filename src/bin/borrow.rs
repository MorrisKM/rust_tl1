#![allow(unused)]

fn main() {
  let s = String::from("rust");
  let s1 = &s;

  //Borrow-temporarily use a value without taking ownership
  //creates a reference (either mutable or immutable)
  //does not move ownership

  //immutable borrow
  //any number of read-only access to a value
  let s = String::from("rust");
  let s1 = &s;
  let s2 = &s;

  //Mutable borrow
  //only one utable reference at a time
  let mut s = String::from("rust");
  let s1 = &mut s;

  //you cannot borrow immutable and mutable simultaneously
  //reference must not outlive the value
}