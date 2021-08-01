// tuples are group of different values
// max 12 values

pub fn run() {
  let person: (&str, &str, i8) = ("Valentin", "Mass", 37);
  println!("{} is from {} and is {}", person.0, person.1, person.2);
}
