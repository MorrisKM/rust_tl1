#![allow(unused)]

//Concurrency
// - Thread
// - Channel
// - Async / await programming

use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
  //a thread returns a Result type sub by JoinHandle
  //concurrency
  let h1: JoinHandle<()> = thread::spawn(|| {
    for i in 0..5 {
      println!("h1 {i}");
      thread::sleep(Duration::from_millis(100));
    }
  });
  let h2: JoinHandle<()> = thread::spawn(|| {
    for i in 0..5 {
      println!("h2 {i}");
      thread::sleep(Duration::from_millis(100));
    }
  });
  h1.join().unwrap();
  h2.join().unwrap();

  //move keyword
  let v = vec![1u32, 2u32, 3u32];
  let h =thread::spawn(move || {
    println!("v: {:?}", v)
  });
  h.join().unwrap();

  //return value
  let h =thread::spawn(|| {
    return 1u32
  });

  match h.join() {
    Ok(val) => println!("v: {val}"),
    Err(err) => {}
  }

  //scoped thread to reference(borrow) data instead of taking ownership
  let msg = "hello".to_string();
  let (v1, v2) = thread::scope(|scope| {
    let h1 = scope.spawn(|| {
      println!("msg 1: {msg}");
      return 1u32;
    });
    let h2 = scope.spawn(|| {
      println!("msg 2: {msg}");
      return 2u32;
    });

    (h1.join().unwrap(), h2.join().unwrap())
  });
  println!("msg: {msg}");
  println!("v1: {v1}, v2: {v2}");
}