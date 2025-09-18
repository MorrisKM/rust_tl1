#![allow(unused)]

fn borrow(s: &[i32]) {
  println!("{:?}", s)
}

fn borrow_mut(s: &mut [i32]) {
  s[0] = -1
}

fn split_at(s: &[i32], i: usize) -> (&[i32], &[i32]){
  (&s[..i], &s[i..])
}
fn main() {
  //borrow and slices
  let a: [i32; 5] = [2, 3, 4, 5, 6];
  let s: &[i32] = &a[0..2];
  borrow(s);

  //mutable slice
  let mut a: [i32;5] = [2, 3, 4, 5, 6];
  let s: &mut[i32] = &mut a[0..2];
  borrow_mut(s);

  //practical
  let a: [i32; 5] = [2, 3, 4, 5, 6];
  let (s0, s1) = split_at(&a, 2);
  println!("{:?}", s0);
  println!("{:?}", s1);
  println!("{:?}", a);
}