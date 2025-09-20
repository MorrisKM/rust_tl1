#![allow(unused)]

fn main() {
  //Vec<T>
  //iter - borrows an immutable ref and returns a iterator that returns &T
  //into_iter - takes ownership and returns a iterator
  //iter_mut - returns a mutable reference of T &mut T

  let vals: Vec<i32> = vec![1, 2, 3];
  //here v is an immutable reference
  for v in vals.iter() {
    println!("{}", v)
  }
  //here the for loop has taken ownership of the vector vals and hence vals cannot the used below the for loop.
  for v in vals.into_iter() {
    println!("{}",v)
  }
  /* this for loop lowkey calls into_iter method and takes ownership of the vector
  for v in vals {
    println!("{}", v)
  }
  */

  //to modify the value, this will not take ownership since iter_mut returns a mutable reference.
  let mut vals: Vec<i32> = vec![1, 2, 3];
  for v in vals.iter_mut() {
    *v += 1;
    println!("{}", v);
  }
}