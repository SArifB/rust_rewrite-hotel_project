#![allow(dead_code)]

use std::{ io::stdin, str::FromStr };

#[derive(Default)]
pub struct User {
  pub name: String,
  pub email: String,
  pub id: u32,
  pub room_choice: u32,
  pub nigths: u32,
  pub discount: u32,
  pub reservation_number: u32,
}

pub fn rin<T: FromStr>() -> T {
  loop {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap_or_default();
    if let Ok(val) = input.trim().parse() {
      return val;
    }
    println!("Invalid input, try again.");
  }
}