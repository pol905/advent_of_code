use std::{io::{BufReader, BufRead}, fs::File, str::FromStr, cmp};

fn calculate_wrapping_paper(length: u64, width: u64, height: u64) -> u64 {
  let top_area = length * width;
  let front_area = length * height;
  let side_area = width * height;
  2 * (top_area + front_area + side_area) + cmp::min(top_area, cmp::min(front_area, side_area))
}


fn calculate_ribbon(length: u64, width: u64, height: u64) -> u64 {
  let min_side = cmp::min(length, cmp::min(width, height));
  let second_min_side;
  if min_side == length {
    second_min_side = cmp::min(width, height);
  } else if min_side == width {
    second_min_side = cmp::min(length, height);
  } else {
    second_min_side = cmp::min(length, width);
  }
  2 * (min_side + second_min_side) + length * width * height
}

fn parse_input(line: String) -> Vec<u64> {
  line.split('x').map(|num| u64::from_str(num).unwrap()).collect()
}

fn main() {
  let file = File::open("./input").unwrap();
  let file_reader = BufReader::new(file);
  let (mut required_wrapping_paper, mut required_ribbon): (u64, u64) = (0,0);
  for line in file_reader.lines() {
    let parsed_input = parse_input(line.unwrap());
    required_wrapping_paper += calculate_wrapping_paper(parsed_input[0], parsed_input[1], parsed_input[2]);
    required_ribbon += calculate_ribbon(parsed_input[0], parsed_input[1], parsed_input[2]);
  }

  println!("{} {}", required_wrapping_paper, required_ribbon);
}

#[test]
fn wrapping_paper_test() {
  let dimensions: Vec<u64> = vec![3,4,5];
  let result = calculate_wrapping_paper(dimensions[0], dimensions[1], dimensions[2]);
  assert_eq!(result, 106);
}

#[test]
fn ribbon_test() {
  let dimensions: Vec<u64> = vec![3,4,5];
  let result = calculate_ribbon(dimensions[0], dimensions[1], dimensions[2]);
  assert_eq!(result, 74);
}