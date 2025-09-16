#[allow(unused)]
#[derive(Debug, PartialEq)]

enum Color {
  Red,
  Green,
  Blue,
  Rgba(u8, u8, u8, f32),
  Hex(String),
  Hsl {h: u8, s: u8, l: u8},
}

fn main() {
  //Enum
  let _color: Color = Color::Red;
  let _color: Color = Color::Green;
  let _color: Color = Color::Rgba(0, 0, 255, 0.1);
  let _color: Color = Color::Hex("#ffffff".to_string());
  let color: Color = Color::Hsl { h: 0, s: 1, l: 200 };
  //Attributes - Debug 
  println!("{:?}", color);
  //PartialEq attribute, this is when trying to compare two enums with each other
  println!("{}", Color::Red == Color::Green);

  //Option = Some(11) | None
  let _x: Option<i32> = None;
  let x: Option<i32> = Some(-11);
  println!("option = {:?}", x);

  //Result = Ok(10) | Err("error msg here")
  let res: Result<u32, String> = Ok(5);
  println!("{:?}", res);
  let res: Result<u32, String> = Err("found no value".to_string());
  println!("{:?}", res);
}