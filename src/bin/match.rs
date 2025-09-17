#![allow(unused)]

enum Animal {
  Cat,
  Dog,
  Duck
}

fn main() {
  let x: i32 = 3;
  //simple match
  match x {
    1 => println!("the value is 1"),
    2 => println!("the value is 2"),
    3 => println!("the value is 3"),
    _ => println!("the value is not 1, 2 or 3")
  }
  //matching multiple fields
  match x {
    1 | 2 | 3 => println!("the value is either 1, 2 or 3"),
    _ => println!("the value is not in range 1 to 3")
  }
  //match in range
  match x {
    1..=10 => println!("the value is between 1 and 10"),
    _ => println!("the value is not in range 1 to 10")
  }

  //getting the exact value from a range match
  match x {
    i @ 1..=10 => println!("the value matched: {i} which is between 1 and 10"),
    _ => println!("the value is not in range 1 to 10")
  }

  //returning a value 
  let animal: Animal = Animal::Duck;
  let animal_matched = match animal {
    Animal::Cat => "it's a cat",
    Animal::Dog => "it's a dog",
    Animal::Duck => "it's a duck",
    _ => "unknown in the enum"
  };
  println!("{}", animal_matched);

  //option
  let x : Option<i32> = Some(5);
  match x {
    Some(v) => println!("Some: {v}"),
    None => println!("a none type")
  }
  //result
  let y: Result<i32, String> = Ok(23);

  match y {
    Ok(v) => println!("Ok: {v}"),
    Err(msg) => println!("No value {msg}")
  }

}