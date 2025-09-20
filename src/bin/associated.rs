#![allow(unused)]

//Associated type
// - Placeholder type inside trait definition
// - Placeholder is replaced by the implementation

//Differenve with generic trait
// - generic = multiple implementation per type
// - assoc type = 1 implementation per type

//Associated type
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}

trait GenericIterator<T> {
  fn get_next(&mut self) -> Option<T>;
}

struct ArrayIter<T> {
  array: [T; 5],
  i: usize,
}

impl GenericIterator<u32> for ArrayIter<u32> {
  fn get_next(&mut self) -> Option<u32> {
    match self.array.get(self.i) {
      Some(v) => {
        self.i += 1;
        Some(*v)
      }
      _ => None,
    }
  }
}

fn main() {
  let mut arr_iter: ArrayIter<u32> = ArrayIter { 
    array: [1, 2, 3, 4, 5], 
    i: 0 
  };
  while let Some(v) = arr_iter.get_next() {
    println!("{:?}", v)
  }
}