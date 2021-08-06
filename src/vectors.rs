// vectors are re-sizable arrays
// import the library mem
use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Get the length
  println!("The length is {}", numbers.len());

  // Add to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value , 6 in this case
  numbers.pop();

  // Re-assign values
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get single val
  println!("Single Value {}", numbers[0]);

  // Vectors are stack allocated
  println!("This vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("Numbers vec: {:?}", numbers);
}
