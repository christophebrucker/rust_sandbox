pub fn run() {

// infinite loop
let mut count = 0;
loop {
  count += 1;
  println!("Number: {}", count);
  if count == 20 {
    break;
  }
}
count = 0;
// while loop (FizzBuzz)
while count <= 100 {
  if count % 15 == 0 {
    println!("fizzbuzz");
  } else if count % 3 == 0 {
    println!("fizz");
  } else if count % 5 == 0 {
    println!("buzz");
  } else {
    println!("{}", count);
  }
  // Increment
  count +=1;
}
count = 0;
// for Range
 for x in 0..100 {
   if x % 15 == 0 {
    println!("fizzbuzz");
  } else if x % 3 == 0 {
    println!("fizz");
  } else if x % 5 == 0 {
    println!("buzz");
  } else {
    println!("{}", x);
  }
  // Increment
  count +=1;
 }

}
