#![allow(unused)]
#[derive(Debug)]
struct Lang {
  language: String,
  version: String,
}
fn main(){
  let lang = "rust";
  println!("hello, {lang}");

  //for multiple variable inside print use indexing
  let x: u32 = 2;
  println!("{0} x {0} = {1}", x, x*x);

  let lang: Lang = Lang {
    language : "rust".to_string(),
    version: "latest".to_string(),
  };

  println!("{:?}", lang);
  println!("{:#?}", lang);
}