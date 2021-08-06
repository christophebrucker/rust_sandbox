// fixed list where the elements are the same datatypes

// import the library mem
use std::mem;

pub fn run() {
  let mut numbers: [i32; 4] = [1, 2, 3, 4];

  // Get the length
  println!("The length is {}", numbers.len());

  // Re-assign values
  numbers[2] = 20;

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value {}", numbers[0]);

  // Arrays are stack allocated
  println!("This array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

}

