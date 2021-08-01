// Primitive String = Immutable fixed lentgth string
// String = Growable string

pub fn run() {
 let mut hello = "Hello"; // Primitive string
 let mut hi = String::from("Hi"); // Growable string

 // Get length
  println!("Length hello: {}", hello.len());

 // Push char
  hi.push(' ');

 // push String
  hi.push_str("World");

  // capacity in bytes
  println!("Capacity: {}", hi.capacity());

  // check if empty
  println!("Empty ?: {}", hi.is_empty());

  // contains substring
  println!("Contains 'World' {}", hi.contains("World") );

  // replace substring
  println!("{}", hi.replace("World", "There"));

  // loop through string by whitespace
  for word in hi.split_whitespace() {
    println! ("{}", word);
  }

  // create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  println!("{}", s);

  // assertion testing
  assert_eq!(3, s.len());
}
