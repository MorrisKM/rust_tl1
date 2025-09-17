#![allow(unused)]

fn main() {
  //loop
  let mut i:i32 = 0;
  loop {
    println!("loop {i}");
    if i == 5 {
      break;
    };
    i += 1
  }

  //while
  let mut j: i32 = 0;
  while j < 3 {
    println!("while {j}");
    j += 1
  }

  //for loop
  for i in 0..=5 { //here if you ommit the = it will execute up to 5 but not including 5
    println!("for loop {}", i)
  }

  //for loop array
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  for i in arr {
    println!("for arr: {i}")
  }
  //using the index
  let i = arr.len();
  for x in 0..i {
    println!("using indexes: {}", arr[x]);
  }

  //returning a value from a loop
  let mut v = 0;
  let z: i32 = loop{
    if v == 4 {
      break 90
    }
    v += 1
  };
  println!("the value of z {}", z);

  //labels
  'outer: for j in 0..5 {
    'inner: for i in 0..5 {
      if j==2 && i==3 {
        println!("j:{j} and i:{i}");
        break 'outer;
      }
    }
  }



}