#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
  //iterator adoptor
  //map, filter, collect, zip, fold
  let vals: Vec<u32> = vec![1,2,3];
  let mut data: Vec<u32> = Vec::new();
  for v in vals.iter() {
    data.push(v *2);
  }
  //rewriting in an iterator adaptor
  let data: Vec<u32> = vals.iter().map(|x| 2 * x).collect();
  let hash: HashSet<u32> = vals.iter().map(|x| x * 2).collect();
  println!("map: {:?}", hash);

  //filter gives a reference into the value being iterated over
  let fil: Vec<u32> = vals.into_iter().filter(|x| *x <= 2).map(|x| x * 2).collect();
  println!("filtered: {:?}", fil);

  //zip combines two collection into key val pairs
  let keys: Vec<String> = vec!["a", "b", "c", "d"].iter().map(|s|s.to_string()).collect();
  let vals: Vec<u32> = vec![1, 2, 3, 4];
  //let zipped: Vec<(String, u32)> = keys.into_iter().zip(vals.into_iter()).collect();
  //zipping to a hashmap
  let zipped: HashMap<String, u32> = keys.into_iter().zip(vals.into_iter()).collect();
  println!("{:?}", zipped);

  //fold is similar to reduce in js and py
  let vals: Vec<u32> = vec![1, 2, 3, 4];
  let sum = vals.iter().fold(0, |acc, x| acc + x);
  println!("sum = {sum}")
  
}