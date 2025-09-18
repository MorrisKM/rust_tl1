#![allow(unused)]

fn main() {
  //Memory -stack and heap
  //Stack
  //stores data of fixed size at compile time
  // Fast
  // LIFO (Last In First Out)

  //Heap
  // Stores data of unknown size at compile time
  // slower than stack
  // Data managed by ownership and borrowing rules

  //ownership rules
  //1. Each value has an owner
  //2. There can only be one owner at a time
  //3. When the owner goes out of scope, the value will be dropped

  //1. Each value has an owner
  //owner of "rust" is s
  let s = String::from("Rust");
  //owner of -1 is i
  let i: i32 = -1;

  //2. There can only be one owner at a time
  let s = String::from("rust");
  //Owner of "rust" changes to s1
  let s1 = s;
  //Owner of "rust" changes to s2
  let s2 = s1;
}