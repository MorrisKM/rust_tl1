use super::foo;
pub fn call_foo() {
  foo::print();
}

pub fn print() {
  println!("my")
}

fn f() {
  a::print();
}

pub mod a;