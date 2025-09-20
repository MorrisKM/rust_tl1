#![allow(unused)]

fn f_fn<F: Fn()> (f: F) {
  f();
  f();
}

fn f_fn_mut<F: FnMut()> (mut f: F) {
  f();
  f();
}

fn f_fn_once<F: FnOnce()> (mut f: F) {
  f();
}


fn main() {
  //Fn (capture by &)
  // - immutable borrow from environment
  // - can be called more than once
  //FnMut (capture by &mut)
  // - mutable borrow from environment
  // - can be called more than once
  //FnOnce (capture by value)
  // - moves captured values into closure, if needed
  // - can be called atleast once, for data types that copy over (primitive types) it can be called more than once

  //Fn (capture by &)
  let s: String = "hello".to_string();
  let f = || println!("fn: {}", s);
  f_fn(f);
  println!("main: {s}");

  //FnMut (capture by &mut)
  let mut v: Vec<i32> = vec![0];
  let mut f = || v.push(0);
  f_fn_mut(f);
  println!("{:?}", v);

  //FnOnce (capture by value)
  let mut v: Vec<i32> = vec![1, 2, 3];
  let f = move|| println!("fn once: {:?}", v);
  f_fn_once(f);
  //this will not work since ownership was moved
  //println!("{:?}", v)
}