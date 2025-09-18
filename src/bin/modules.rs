#![allow(unused)]


//Modules

use first::my;

fn main() {
  my::print();
  my::a::print();
  let s = my::a::S{id: 1, name: "hello".to_string()};
  println!("{:?}", s);
}