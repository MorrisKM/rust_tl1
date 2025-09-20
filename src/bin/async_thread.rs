#![allow(unused)]

//Async vs thread
//native thread
// - good for CPU bound computations
// - limited by memory or OS thread limits

use std::time::Duration;

//aync programming 
// - lower memory consumption
// - no limit on number of threads (spawn virtual threats managed by async runtime (green threads)) can spawn thousands of threads without crashing.
// - good for IO bound computations examples: (network requests and reading files)
#[tokio::main]
async fn main() {
  //native threads 
  let mut handles = vec![];
  // this spawn a million native threads 
  for i in 0..1000000 {
    let handle = std::thread::spawn(move || {
      std::thread::sleep(Duration::from_millis(100));
      println!("native thread {i}");
    });
    handles.push(handle);
  }
  //executes and crashes due to threads limit in the OS
  for h in handles {
    h.join();
  }

  //async 
  let mut handles = vec![];
  // this spawn a million async threads
  for i in 0..1000000 {
    let fut = async move {
      tokio::time::sleep(Duration::from_millis(100));
      println!("native thread {i}");
    };

    let handle = tokio::task::spawn(fut);
    handles.push(handle);
  }

  //executes to the last digit
  for h in handles {
    h.await;
  }
}