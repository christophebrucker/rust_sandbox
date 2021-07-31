/* Primitive types are
 Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 the number is the number of bytes they take in the memory)
 Floats: f32, f64
 Boolean (bool)
 Charactes (char)
 Tuples
 Arrays */

 /* Rust is a statically typed language, which means that
 it must know the types of all variables at compile time.
 However, the compiler can generally infer what type we
 want to use considering the value and how we use it.*/

pub fn run() {

  // by default is i32
  let x = 1;

  // by default is f64
  let y = 2.5;

  // add explicit type
  let z: i64 = 454545454545454;

  // find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // boolean
  let is_active: bool = true;

  // get a boolean from an expression
  let is_greater: bool = 10 > 5;

  // characater char
  let a = 'a';
  let face = '\u{1F600}';
  // slash and backslash /\
  println!("{:?}",( x, y, z, is_active, is_greater, a, face));
}
