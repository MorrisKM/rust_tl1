use std::i32;

#[allow(unused)]

fn main() {
  //signed integers -(2**(n-1) to 2**(n-1) -1)
  let i0: i8 = 1; //from -128 to 127
  let i1: i16 = 1;
  let i2: i32 = 1;
  let i3: i64 = 1;
  let i4: i128 = 1;
  let i5: isize = 1; //based on the processor achitecture 32bit or 64bit
  //unsigned integers the changes the sign to u from value 0 to 2**n - 1

  //floats 
  let f0: f32 = 0.01;

  //boolean 
  let b: bool = true;

  //characters take in letter and also emoji's
  let c: char = 'c';

  //type conversion
  let i: i32 = 1;
  let u: u32 = i as u32;
  let x: u32 = u + (i as u32);

  //min and max for scalars that can be compared using > or <
  let min_i: i32 = i32::MIN;
  let max_i: i32 = i32::MAX;

  //overflow
  let mut u: u32 = u32::MAX;
  //this code will panic in dev mode because of the overflow, it will give a val 0 in production mode
  // u += 1;
  // println!("overflow u32: {u}");
 
  //check_add - Some(x) | None  this returns None if it was (1:1) it would return Some(2)
  let u : Option<u32> = u32::checked_add( u32::MAX, 1);
  println!("checked_add: {:?}", u);

  //wrapping_add   this returns 0 just like normal add
  let u = u32::wrapping_add(u32::MAX, 1);
  println!("wrapping_add: {:?}", u);

}