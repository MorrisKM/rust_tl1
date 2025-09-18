#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

fn main(){
  //initialize
  let mut scores: HashMap<String, u32> = HashMap::new();

  //insert
  scores.insert("red".to_string(), 100);
  scores.insert("green".to_string(), 100);

  //get also returns an option
  let val: Option<&u32> = scores.get("green");
  println!("val: {:?}", val);

  //updating is simply overriding the value
  scores.insert("red".to_string(), 50);

  //upsert -update if value exist or insert if it does not exist
  //this will insert blue since it does not exist
  let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
  *v += 200;

  let val: Option<&u32> = scores.get("blue");
  println!("val: {:?}", val);

  //this will update the value of blue since it does exist return 400
  let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
  *v += 200;

  let val: Option<&u32> = scores.get("blue");
  println!("val: {:?}", val);
}