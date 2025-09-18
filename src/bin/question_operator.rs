#![allow(unused)]

fn f1() -> Result<u32, String> {
  println!("f1");
  Ok(1)
}
fn f2() -> Result<u32, String> {
  println!("f2");
  Ok(2)
}

fn f_match() -> Result<u32, String> {
  let res1 = f1();
  let x1 = match res1 {
    Ok(val) => val,
    Err(err) => {
      return Err(err)
    }
  };
  let res2 = f2();
  let x2 = match res2 {
    Ok(val) => val,
    Err(err) => return Err(err)
  };
  Ok(x1 + x2)
}

fn f_question() -> Result<u32, String> {
  let x1 = f1()?;
  let x2 = f2()?;
  Ok(x1 + x2)
}

fn main() -> Result<(), String> {
  let x = f1()?;
  println!("x = {x}");
  let z = f_question();
  match z {
    Ok(x) => println!("z = {x}"),
    Err(err) => println!("err {err}")
  }

  Ok(())
}