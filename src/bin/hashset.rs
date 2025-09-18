#![allow(unused)]

use std::collections::HashSet;

fn main(){
  //Hashset - generic collections(can take types) HashSet<u32>
  let mut set: HashSet<u32> = HashSet::new();
  //inserting data into the hashset
  //the function insert returns a boolean 
  let inserted: bool = set.insert(1);
  println!("inserted: {inserted}");
  
  let inserted: bool = set.insert(1);
  println!("inserted: {inserted}");

  //contains returns a bool and the value passed should be a reference, follow the linters
  let contains: bool = set.contains(&1);
  println!("contains 1?: {contains}");

  let contains: bool = set.contains(&2);
  println!("contains 2?: {contains}");
}