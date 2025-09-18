#![allow(unused)]

fn take(s:String) {
  println!("take {s}")
}

fn borrow_imm(s: &str) {
  println!("borrow immutable {s}")
}

fn borrow_mm(s: &mut String) {
  s.push_str("language");
}

fn print_len(s:String) {
  println!("take {}", s.len())
}

fn print_len_return_ownership(s:String) -> String {
  println!("return ownership length {}", s.len());
  s
}

fn print_len_borrow(s: &str) {
  println!("borrow length {}", s.len())
}

fn main(){
  //Take ownership
  let s:String = String::from("Rust");
  take(s); //the function take takes ownership of s
  //this print will not work
  //println!("{s}");

  //Borrow immutable
  let s: String = String::from("Rust");
  borrow_imm(&s);
  println!("{s}");

  //Borrow mutable
  let mut s: String = String::from("Rust");
  borrow_mm(&mut s);
  println!("{s}");

  //Modify a function 
  //1. Take ownership
  let s:String = String::from("Rust");
  print_len(s); //the function take takes ownership of s

  //2. Returns ownership
  let s:String = String::from("Rust");
  let s = print_len_return_ownership(s); 
  println!("{s}");

  //3. Borrows
  let s:String = String::from("Rust");
  print_len_borrow(&s); 
  println!("{s}")


}