use std::str::FromStr;

#[cfg(test)]
#[path="./submarine.test.rs"]
mod submarine_test;

#[derive(Debug, Clone, Copy)]
pub struct Submarine {
  horizontal: isize,
  depth: isize,
  aim: isize,
}

#[derive(Debug, PartialEq)]
pub enum Command {
  Down(usize),
  Up(usize),
  Forward(usize),
}


impl FromStr for Command {
  type Err = &'static str;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    let mut words = string.split(' ').take(2);
    let (prefix, arg) = (words.next().unwrap(), words.next().unwrap().parse::<usize>().unwrap());
    match prefix {
      "down" => Ok(Self::Down(arg)),
      "up" => Ok(Self::Up(arg)),
      "forward" => Ok(Self::Forward(arg)),
      &_ => Err("Unknown error type")
    }

  }
}

impl Submarine {
  pub fn new() -> Self {
    Self {
      depth: 0,
      horizontal: 0,
      aim: 0,
    }
  }

  pub fn position_product(&self) -> isize {
    self.depth * self.horizontal
  }

  pub fn perform_command(&self, command: &Command) -> Self {
    match command {
      Command::Down(amount) => Self { depth: self.depth + (*amount as isize), ..*self },
      Command::Up(amount) => Self { depth: self.depth - (*amount as isize), ..*self },
      Command::Forward(amount) => Self { horizontal: self.horizontal + (*amount as isize), ..*self },
    }
  }

  pub fn perform_command_with_aim(&self, command: &Command) -> Self {
    match command {
      Command::Down(amount) => Self { aim: self.aim + (*amount as isize), ..*self },
      Command::Up(amount) => Self { aim: self.aim - (*amount as isize), ..*self },
      Command::Forward(amount) => Self {
        horizontal: self.horizontal + (*amount as isize),
        depth: self.depth + self.aim * (*amount as isize),
        ..*self
      },
    }
  }

  pub fn perform_commands(&self, commands: &[Command], with_aim: bool) -> Self {
    commands.iter()
      .fold(*self, |pos, command| if with_aim {
          pos.perform_command_with_aim(command)
        } else {
          pos.perform_command(command)
        })
  }
}
