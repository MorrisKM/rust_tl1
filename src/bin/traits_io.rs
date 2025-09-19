#![allow(unused)]

trait Animal {
  fn speak(&self) -> String;
}

struct Cat;
struct Dog;

impl Animal for Cat {
  fn speak(&self) -> String {
      "meow".to_string()
  }
}

impl Animal for Dog {
  fn speak(&self) -> String {
      "woof".to_string()
  }
}
//static dispatch - trait is known at compile time, this one take in a ref to implementation of Anima
fn greet(animal: &impl Animal) {
  println!("static {}", animal.speak());
}

//dynamic dispatch - trait is not known at compile time
fn greet_dyn(animal: &dyn Animal) {
  println!("dynamic {}", animal.speak());
}

//this one returns a struct also known at compile time
fn return_concretet_type() -> impl Animal {
  Dog
}

fn rand_animal(rand: u32) -> Box<dyn Animal> {
  if rand <= 10 {
    Box::new(Dog)
  } else {
    Box::new(Cat)
  }
}

fn main() {
  let cat = Cat;
  let dog = Dog;

  greet(&cat);
  greet(&dog);

  let animal = return_concretet_type();
  println!("animal.speak: {}", animal.speak());

  let animal_str = "dog";
  let animal: &dyn Animal = match animal_str {
      "dog" => &Dog,
      _ => &Cat
  };
  greet_dyn(animal);

  let animal = rand_animal(11);
  println!("rand animal: {}", animal.speak());
}