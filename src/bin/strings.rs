#[allow(unused)]

fn main() {
  //String = vector of u8 (Vec<u8>) valid UTF-8

  //&str = slice of u8 (&[u8]) valid UTF-8
  //When to use String vs &str
  //String -> mutate or data needs to be owned
  //&str -> read only

  //String
  let msg: String = String::from("Hello Rust");
  //this gets the length of the string
  let len: usize = msg.len();

  //&str  -usually used str with reference (borrowed)
  //immutable
  let s: &str = &msg[0..5];
  let len: usize = s.len();
  println!("slice: {s}");

  //string literals are stored inside binary
  //slice pointing to a specific part of the binary
  //immutable because hard-coded inside binary
  let hello: &str = "Hello Rust";

  //multiline string literal
  let s: &str = r#"
    {"a" : 1,
     "b" : {"c" : 2}
    }
  "#;
  println!("{s}");

  //deref coercion
  let msg: String = String::from("Hello Rust");
  let s: &str = &msg;

  //mutating a string 
  //add &str to String
  let mut msg: String = "Hello Rust".to_string();
  msg += "!";

  //string interpolation, mixing string literals with variables
  let lang = "Rust";
  let msg = format!("hello {lang}");  //same as what you can do with println!()
}