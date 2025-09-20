#![allow(unused)]
//multi producer single consumer
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;
fn main() {
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

  for i in 0..3 {
    let tx_clone = tx.clone();
    thread::spawn(move ||{
      thread::sleep(Duration::from_millis(1000));
      tx_clone.send(format!("hello {i}")).unwrap()
    });
  }

  //dropping sender to signal recv that there is no more messages to be received
  std::mem::drop(tx);

  loop{
    match rx.recv() {
      Ok(msg) => println!("res: {:#?}", msg),
      Err(err) => {
        println!("err: {:#?}", err);
        break;
      }
    }
  }
}