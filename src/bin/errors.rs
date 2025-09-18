#![allow(unused)]
#[derive(Debug)]
enum MathError {
  DivByZero,
  Other
}

fn divide(x: i32, y:i32) -> Result<i32, MathError> {
  if x == 0 {
    return Err(MathError::DivByZero);
  }
  Ok(y/x)
}
fn main(){
  let arr: [i32; 3] = [2, 3, 4];
  let x: Option<&i32> = arr.get(9);
  match x {
    Some(val) => println!("some : {val}"),
    None => println!("none")
  }

  let x: i32 = 4;
  let y: i32 = 3;
  //result
  let z = divide(x, y);
  match z {
    Ok(val) => println!("result: {val}"),
    Err(err) => println!("error: {:?}", err)
  }
}