#![allow(unused)]

use std::cell::{RefCell, RefMut};

//interior mutability
// - Allows data mutation even when there are immutable references to that data
//refcell
// - Run time error when borrowing rules are broken, only one mutable reference at a time
// - Single threaded use
// - Used in combination with Rc to create a mutable data with shared ownership 

fn main() {
  let s = String::from("rust");
  let r: RefCell<String> = RefCell::new(s);
  { 
    //introduced scope to drop RefMut which is mut r1
    let mut r1: RefMut<'_, String> = r.borrow_mut();
    *r1 += "language";
    println!("r: {:#?}", r)
  }
  println!("r: {:#?}", r)
}