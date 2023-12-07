use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// fn main() -> std::io::Result<()> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
  part1()?;
  part2()?;

  Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
  let string_to_numbers = build_string_to_numbers_hashmap();
  let mut sum: u32 = 0;

  let reader = line_reader()?;
  for line in reader.lines() {
    let line = line?;
    let (first_digit, last_digit) = process_line(&line, &string_to_numbers)?;
    sum = sum + (first_digit * 10 + last_digit);
  }

  println!("{}", sum);

  Ok(())
}

fn process_line(
  line: &String,
  string_to_numbers: &HashMap<&str, u32>,
) -> Result<(u32, u32), Box<dyn std::error::Error>> {
  let mut biggest_found_index_and_number: Option<(usize, u32)> = None;
  let mut smallest_found_index_and_number: Option<(usize, u32)> = None;

  for (digit_name, digit) in string_to_numbers {
    let indices: Vec<usize> = line
      .match_indices(digit_name)
      .map(|(index, _)| index)
      .collect();

    match indices.last() {
      Some(last_index) => match biggest_found_index_and_number {
        None => {
          biggest_found_index_and_number = Some((*last_index, *digit));
        }
        Some((idx, _d)) => {
          if idx < *last_index {
            biggest_found_index_and_number = Some((*last_index, *digit));
          }
        }
      },
      None => {}
    }

    match indices.first() {
      Some(first_index) => match smallest_found_index_and_number {
        None => {
          smallest_found_index_and_number = Some((*first_index, *digit));
        }
        Some((idx, _d)) => {
          if idx > *first_index {
            smallest_found_index_and_number = Some((*first_index, *digit));
          }
        }
      },
      None => {}
    }
  }

  let digit_one = smallest_found_index_and_number.ok_or("digit not found")?;
  let digit_two = biggest_found_index_and_number.ok_or("digit not found")?;

  // println!("{}", line);
  // println!("{} {}", digit_one.1, digit_two.1);

  Ok((digit_one.1, digit_two.1))
}

fn build_string_to_numbers_hashmap() -> HashMap<&'static str, u32> {
  let mut string_to_numbers = HashMap::new();

  string_to_numbers.insert("one", 1);
  string_to_numbers.insert("two", 2);
  string_to_numbers.insert("three", 3);
  string_to_numbers.insert("four", 4);
  string_to_numbers.insert("five", 5);
  string_to_numbers.insert("six", 6);
  string_to_numbers.insert("seven", 7);
  string_to_numbers.insert("eight", 8);
  string_to_numbers.insert("nine", 9);

  string_to_numbers.insert("1", 1);
  string_to_numbers.insert("2", 2);
  string_to_numbers.insert("3", 3);
  string_to_numbers.insert("4", 4);
  string_to_numbers.insert("5", 5);
  string_to_numbers.insert("6", 6);
  string_to_numbers.insert("7", 7);
  string_to_numbers.insert("8", 8);
  string_to_numbers.insert("9", 9);

  string_to_numbers
}

fn line_reader() -> Result<BufReader<File>, Box<dyn std::error::Error>> {
  let path = Path::new("./1.txt");
  let file = File::open(path)?;
  Ok(BufReader::new(file))
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
  let mut sum: u32 = 0;

  let reader = line_reader()?;
  for line in reader.lines() {
    let line = line?;

    // println!("{}", line);

    let first_digit_char = line
      .chars()
      .find(|c| c.is_digit(10))
      .ok_or("first digit not found")?;
    let last_digit_char = line
      .chars()
      .rev()
      .find(|c| c.is_digit(10))
      .ok_or("last digit not found")?;

    let first_digit = first_digit_char.to_digit(10).ok_or("oops")?;
    let last_digit = last_digit_char.to_digit(10).ok_or("oops")?;

    sum = sum + (first_digit * 10 + last_digit);
  }

  println!("{}", sum);

  Ok(())
}
