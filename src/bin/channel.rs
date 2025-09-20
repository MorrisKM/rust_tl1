#![allow(unused)]
//multi producer single consumer
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;
fn main() {
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
  //send returns a result since it can return an error if you try to send to a dropped receiver  
  //std::mem::drop(rx); will result to a SendError
  thread::spawn(move || {
    thread::sleep(Duration::from_millis(1000));
    tx.send("hello 1".to_string()).unwrap();
    tx.send("hello 2".to_string()).unwrap();
    tx.send("hello 3".to_string()).unwrap();
    tx.send("hello 4".to_string()).unwrap();
  });

  //recv will block the main thread until the message is received so you dont have to unwrap result from the Joinhandle.
  //let res = rx.recv();
  //println!("res: {:#?}", res);
  //for a non-blocking handle use try_recv
  loop {
    match rx.try_recv() {
        Ok(msg) => println!("res: {:#?}", msg),
        Err(TryRecvError::Empty) => println!("no message"),
        Err(TryRecvError::Disconnected) => {
          println!("diconnected");
          break;
        }
    };
    thread::sleep(Duration::from_millis(200));
  };


  //Dropping the sender
  
  
}