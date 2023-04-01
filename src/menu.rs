#![allow(dead_code)]

use console::Term;
use std::io;

#[derive(Default)]
pub enum Menu {
  #[default] Main,
  Side1,
  Side2,
  Side3,
  End,
  Error,
}

impl Menu {
  pub fn get_command() -> char {
    loop {
      // Create a buffer and read new input
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap_or_default();
      // Clear the screen and return input
      if let Ok(val) = input.trim().parse() {
        Self::clear_screen();
        return val;
      }
      println!("Invalid input, try again.");
    }
  }

  pub fn clear_screen() {
    Term::stdout().clear_screen().expect("failed to clear the screen");
  }

  //--------------------------------------------------------------------------------------------
  // The number in function name indicates the amount of choices for user switching

  // Switch Self without command
  pub fn switch0(&mut self, cond: Self) {
    *self = cond;
    Self::clear_screen();
  }

  pub fn switch1(&mut self, cond1: Self) {
    // Read and match input to command
    *self = match Self::get_command() {
      '1' => cond1,
      'l' | 'L' => Self::End,
      _ => Self::Error,
    };
  }

  pub fn switch2(&mut self, cond1: Self, cond2: Self) {
    // Read and match input to command
    *self = match Self::get_command() {
      '1' => cond1,
      '2' => cond2,
      'l' | 'L' => Self::End,
      _ => Self::Error,
    };
  }

  pub fn switch3(&mut self, cond1: Self, cond2: Self, cond3: Self) {
    // Read and match input to command
    *self = match Self::get_command() {
      '1' => cond1,
      '2' => cond2,
      '3' => cond3,
      'l' | 'L' => Self::End,
      _ => Self::Error,
    };
  }

  pub fn switch4(
    &mut self,
    cond1: Self,
    cond2: Self,
    cond3: Self,
    cond4: Self
  ) {
    // Read and match input to command
    *self = match Self::get_command() {
      '1' => cond1,
      '2' => cond2,
      '3' => cond3,
      '4' => cond4,
      'l' | 'L' => Self::End,
      _ => Self::Error,
    };
  }
}