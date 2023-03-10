use std::{io::{BufReader, BufRead}, fs::File, str::FromStr, cmp};

fn calculate_wrapping_paper(length: u64, width: u64, height: u64) -> u64 {
  let top_area = length * width;
  let front_area = length * height;
  let side_area = width * height;
  2 * (top_area + front_area + side_area) + cmp::min(top_area, cmp::min(front_area, side_area))
}

fn parse_input(line: String) -> Vec<u64> {
  line.split('x').map(|num| u64::from_str(num).unwrap()).collect()
}

fn main() {
  let file = File::open("./input").unwrap();
  let file_reader = BufReader::new(file);
  let mut required_wrapping_paper: u64 = 0;
  for line in file_reader.lines() {
    let parsed_input = parse_input(line.unwrap());
    required_wrapping_paper += calculate_wrapping_paper(parsed_input[0], parsed_input[1], parsed_input[2]);
  }

  println!("{}", required_wrapping_paper);
}
