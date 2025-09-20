#![allow(unused)]
//async / await

async fn g1() -> u32 {
  println!("g1");
  1
}
async fn g2() -> u32 {
  println!("g2");
  2
}
async fn g3() -> u32 {
  println!("g3");
  3
}

async fn f() {
  println!("f");
  //tokio join will execute the future returned by the async nature of the functions, these functions g1 ang g2 will execute simultaneously
  let (r1, r2) = tokio::join!(g1(), g2());
  // let r1 = g1().await;
  // let r2 = g2().await;
  let r3 = g3().await;
  println!("{r1} {r2} {r3}");
}

//async runtime is needed to co-ordinate and schedule how async functions will execute
#[tokio::main]
async fn main() {
  f().await;
}