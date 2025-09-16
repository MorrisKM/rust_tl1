#[allow(unused)]

fn main() {
  //Array - fixed length, known at compile time
  let arr: [u32; 3] = [1, 2, 3];
  //accessing items in an array
  println!("arr[2] = {}", arr[2]);

  //mutating an array
  let mut arr: [u32; 3] = [1, 2, 3];
  arr[1] = 9;

  //creating an array with all equal items, will have a length of ten all numbers of value 0
  let arr: [u32; 10] = [0 ; 10];


  //Slice - length not known at compile time
  let nums: [i32; 10] = [-1, 2, -2, 2, -3, 3, -4, 4, -5, 5];

  //first 3 elements
  let s = &nums[0..3]; // 0, 1, 2 not including the third index
  //all elements
  let y: &[i32] = &nums[..];
}