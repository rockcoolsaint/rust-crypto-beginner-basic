// Can't overwrite an immutable var
fn immutable() {
  let x = 5;
  x = 6;
}

// Must overwrite with let
fn overwrite() {
  let x = 5;
  let x = 6;
}

// Can't mutate a variable's type
fn mutate_type() {
  let some_string = "donut";
  some_string = some_string.len();
}

// Must mutate with "let"
fn overwrite() {
  let some_string = "donut";
  let some_string = some_string.len();
}

// Declare data types
fn Declare() {
  let x: u32 = 20;
}

// Intgers
fn integers() {
  let Signed_int: i32 = 34;
  let unsigned_int: u32 = 11; 
}

// Booleans
fn booleans() {
  let t: bool = true;
  let f: bool = false;
}

// Strings
fn Strings() {
  let some_slice: &str = "donut"; // size
  let some_string: String = String::from("donut"); String
}

// Constants
const SOME_CONSTANT: u32 = 20;

fn main() {
    println!("Hello, world!");
}
