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
  for (pos, instruction) in instructions.chars().enumerate() {
    if current_floor.last() == Some(&patterns[&instruction]) {
      current_floor.pop();
    } else {
      if instruction == ')' {
        return (pos + 1) as i64;
      }
      current_floor.push(instruction);
    }
  }
  -1
}

fn main() {
  let mut file = File::open("./input").unwrap();
  let mut buf = String::from("");
  file.read_to_string(&mut buf).unwrap();
  println!("{}", find_floor(String::from(buf.trim_end())));
}