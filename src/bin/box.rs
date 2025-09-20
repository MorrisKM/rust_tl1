#![allow(unused)]

//Smart pointer
//Pointer with metadata and additional capabilities

//Box
// - Allows data to be stored on the heap
// - Useful for data where the size is not known at compile time
//      - Trait objects
//      - Recursive data structure

use std::fs::File;
use std::io::{self, Read};

//the return value of Err() is dynamic, two possible errors can be returned and since the size of trait object is not known at compile time it is wrapped in a Box.
fn read_file(path: &str) -> Result<i32, Box<dyn std::error::Error> > {
  let mut file = File::open(path)?; // std::io::Error
  let mut data = String::new();
  file.read_to_string(&mut data)?; // std::io::Error
  let num: i32 = data.parse()?; // std::num::ParseIntError
  Ok(num)
}

//recursive data structure
#[derive(Debug)]
struct Tree {
  val: i32,
  left: Option<Box<Tree>>,
  right: Option<Box<Tree>>
}

fn main() {
  let i: i32 = 1;
  let b: Box<i32> = Box::new(i);
  let v = *b;


  let tree = Tree {
    val: 1,
    left: Some(Box::new(Tree {
      val: 2,
      left: None,
      right: Some(Box::new(Tree {
        val: 3,
        left: None,
        right: None
      }))
    })),
    right: Some(Box::new(Tree {
      val: 4,
      left: None,
      right: None
    }))
  };

  println!("{:#?}", tree);
  println!("tree.left.right.val: {}", tree.left.unwrap().right.unwrap().val)
}