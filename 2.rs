use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  part1()?;
  part2()?;

  Ok(())
}

#[derive(Debug)]
struct CubeSet {
  red: u8,
  blue: u8,
  green: u8,
}

impl CubeSet {
  fn new() -> CubeSet {
    CubeSet {
      red: 0,
      blue: 0,
      green: 0,
    }
  }
}

const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;
const BLUE_CUBES: u8 = 14;

impl CubeSet {
  fn is_possible(&self) -> bool {
    self.red <= RED_CUBES && self.green <= GREEN_CUBES && self.blue <= BLUE_CUBES
  }
}

#[derive(Debug)]
struct Game {
  game_number: u8,
  sets_of_cubes: Vec<CubeSet>,
}

impl Game {
  fn power(&self) -> u32 {
    let max_red = self.sets_of_cubes.iter().map(|s| s.red).max().unwrap();
    let max_green = self.sets_of_cubes.iter().map(|s| s.green).max().unwrap();
    let max_blue = self.sets_of_cubes.iter().map(|s| s.blue).max().unwrap();

    (max_red as u32) * (max_green as u32) * (max_blue as u32)
  }
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
  let games = init_all_games()?;

  let sum_of_ids: u32 = games
    .iter()
    .filter(|g| g.sets_of_cubes.iter().all(|s| s.is_possible()))
    .map(|g| g.game_number as u32)
    .sum();

  println!("{}", sum_of_ids);

  Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
  let games = init_all_games()?;

  let power: u32 = games.iter().map(|g| g.power()).sum();

  println!("{}", power);

  Ok(())
}

fn init_all_games() -> Result<Vec<Game>, Box<dyn std::error::Error>> {
  let reader = line_reader()?;

  let mut games: Vec<Game> = Vec::new();

  for line in reader.lines() {
    let line = line?;
    let game = parse_line(line.as_str())?;

    games.push(game);
  }

  Ok(games)
}

fn parse_line(line: &str) -> Result<Game, Box<dyn std::error::Error>> {
  let chunks: Vec<&str> = line.splitn(2, ':').collect();

  let game_declaration = *(chunks
    .get(0)
    .ok_or("failed to extract the game declaration")?);
  let sets_of_cubes = *(chunks
    .get(1)
    .ok_or("failed to extract the description of cube sets")?);

  let game_number = extract_game_number(game_declaration)?;

  let game = Game {
    game_number,
    sets_of_cubes: Vec::new(),
  };

  let game = parse_game_string_declaration(game, sets_of_cubes)?;

  Ok(game)
}

fn extract_game_number(game_declaration: &str) -> Result<u8, Box<dyn std::error::Error>> {
  let re = Regex::new(r"Game (\d+)").unwrap();

  let game_number = re
    .captures(game_declaration)
    .and_then(|cap| cap.get(1).unwrap().as_str().parse::<u8>().ok())
    .ok_or("oops")?;

  Ok(game_number)
}

fn parse_game_string_declaration(
  mut game: Game,
  sets_of_cubes: &str,
) -> Result<Game, Box<dyn std::error::Error>> {
  let re = Regex::new(r"(\d+) (\w+)").unwrap();

  let sets: Vec<&str> = sets_of_cubes.split(';').collect();

  for set in sets.iter() {
    let cube_set = parse_set_string_declaration(&re, set)?;
    game.sets_of_cubes.push(cube_set);
  }

  Ok(game)
}

fn parse_set_string_declaration(
  re: &Regex,
  set_declaration: &str,
) -> Result<CubeSet, Box<dyn std::error::Error>> {
  let mut cube_set = CubeSet::new();

  for cap in re.captures_iter(set_declaration) {
    let n = cap.get(1).unwrap().as_str().parse::<u8>()?;
    let color = cap.get(2).unwrap().as_str();

    match color {
      "red" => cube_set.red = n,
      "blue" => cube_set.blue = n,
      "green" => cube_set.green = n,
      _ => (),
    }
  }
  Ok(cube_set)
}

fn line_reader() -> Result<BufReader<File>, Box<dyn std::error::Error>> {
  let path = Path::new("./2.txt");
  let file = File::open(path)?;
  Ok(BufReader::new(file))
}
