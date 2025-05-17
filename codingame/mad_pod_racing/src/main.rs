use std::{io, ops::Div};

macro_rules! parse_input {
  ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const MAX_WIDTH: i32 = 16000;
const MAX_HEIGHT: i32 = 9000;
const CHECKPOINT_RADIUS: i32 = 600;

struct Position(i32, i32);

struct NextCheckpoint {
  pub position: Position,
  pub angle: i32,
  pub dist: i32
}

fn slow_thrust(distance: i32, close_enough: i32) -> i32 {
  // For small distances, deccelerate as getting closer
  let cnt = (100.0).div(close_enough.pow(4) as f32);
  println!("cnt = {cnt}");
  return (cnt * (distance.pow(4) as f32)) as i32;
}

fn calculate_thrust(curr_pos: Position, next_checkpoint: NextCheckpoint) -> i32 {
  let close_enough = 1000;

  if next_checkpoint.dist < close_enough {
    if next_checkpoint.angle.abs() < 45 {
      return slow_thrust(next_checkpoint.dist, close_enough);
    }
    return 80;
  }
  100
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
  let ce: i32 = 1500;
  let ce4 = (ce as i64).pow(3);
  println!("1500^4 = {ce4}");
  let st = slow_thrust(1200, 1500);
  println!("slow_thrust(1200, 1500) = {st}");
  // game loop
  // loop {
  //   let mut input_line = String::new();
  //   io::stdin().read_line(&mut input_line).unwrap();
  //   let inputs = input_line.split(" ").collect::<Vec<_>>();
  //   let x = parse_input!(inputs[0], i32);
  //   let y = parse_input!(inputs[1], i32);
  //   let next_checkpoint_x = parse_input!(inputs[2], i32); // x position of the next check point
  //   let next_checkpoint_y = parse_input!(inputs[3], i32); // y position of the next check point
  //   let next_checkpoint_dist = parse_input!(inputs[4], i32); // distance to the next checkpoint
  //   let next_checkpoint_angle = parse_input!(inputs[5], i32); // angle between your pod orientation and the direction of the next checkpoint
  //   let mut input_line = String::new();
  //   io::stdin().read_line(&mut input_line).unwrap();
  //   let inputs = input_line.split(" ").collect::<Vec<_>>();
  //   let opponent_x = parse_input!(inputs[0], i32);
  //   let opponent_y = parse_input!(inputs[1], i32);

  //   // Write an action using println!("message...");
  //   // To debug: eprintln!("Debug message...");

  //   // You have to output the target position
  //   // followed by the power (0 <= thrust <= 100)
  //   // i.e.: "x y thrust"
  //   let thrust = calculate_thrust(Position(x,y), NextCheckpoint {
  //     position: Position(next_checkpoint_x, next_checkpoint_y),
  //     angle: next_checkpoint_angle,
  //     dist: next_checkpoint_dist
  //   });

  //   println!("{next_checkpoint_x} {next_checkpoint_y} {thrust}");
  // }
}
