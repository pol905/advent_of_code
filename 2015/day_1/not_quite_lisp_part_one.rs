use std::{
  collections::HashMap,
  fs::File,
  io::Read
};

fn find_floor(instructions: String) -> i64 {
  let mut current_floor = vec![];
  let patterns = HashMap::from([
    ('(', ')'),
    (')', '('),
  ]);
  for instruction in instructions.chars() {
    if current_floor.last() == Some(&patterns[&instruction]) {
      current_floor.pop();
    } else {
      current_floor.push(instruction);
    }
  }
  let current_floor_len = current_floor.len();

  if current_floor.last() == Some(&')') {
    return (0 - current_floor_len) as i64;
  }

  current_floor_len as i64
}

fn main() {
  let mut file = File::open("./input").unwrap();
  let mut buf = String::from("");
  file.read_to_string(&mut buf).unwrap();
  println!("{}", find_floor(String::from(buf.trim_end())));
}