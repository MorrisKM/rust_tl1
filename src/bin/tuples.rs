#[allow(unused)]

fn main() {
  // Tuple
  let t: (bool, u32, char) = (true, 1, 'c');

  //accessing values
  println!("t = {}, {}, {}", t.0, t.1, t.2);

  //destructure
  let (a, b, c) = t;

  //ignore with _
  let (_, b, _) = t;

  //Empty tuple - unit type
  let t = ();

  //Nested tuple
  let nested = ((1.23, 'a'), (true, 1, 'b'), ());
}