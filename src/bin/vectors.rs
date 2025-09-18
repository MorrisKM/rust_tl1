#![allow(unused)]

fn main() {
  //Vect<T> like array but are dynamic, their size can grow and shrink at run time
  let v: Vec<i32> = vec![-1, 2, 1];
  let v: Vec<u32> = vec![1, 2, 3];
  //initializing an empty vector
  let v: Vec<i32> = Vec::new();

  let v = vec![1u8, 2, 3];
  let v = vec![1u8, 1, 1, 1, 1];
  let v = vec![1u8; 5];

  println!("v = {:?}, length = {}", v, v.len());

  //get
  let v: Vec<i32> = vec![1, 2, 3];
  //if the vector v is empty this will panic and crash the main function
  let x: i32 = v[0];
  //to avoid the crash use 
  let x: Option<&i32> = v.get(0);
  match x {
    Some(val) => println!("val = {val}"),
    None => println!("invalid index")
  }

  //update
  let mut v: Vec<i32> = vec![1, 2, 3];
  v[1] = 4;

  //push
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  println!("{:?}", v);

  //pop returns an option
  let mut v: Vec<i32> = vec![1, 2, 3, 4];
  match v.pop(){
    Some(val) => println!("pop: {:?}",val),
    None => println!("Pop: v is none")
  }

  //slice
  let v: Vec<i32> = vec![1, 2, 3, 4, 5];
  let s: &[i32] = &v[1..4];
  println!("{:?}",s)

}