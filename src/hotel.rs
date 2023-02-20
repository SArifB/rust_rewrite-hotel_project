#![allow(dead_code)]

use rand::{ distributions::Uniform, thread_rng, Rng };
use std::collections::HashSet;

pub struct Hotel {
  total_size: u32,
  availability: Vec<bool>,
  capacity: Vec<u32>,
  price: Vec<u32>,
  reservation_num: Vec<u32>,
  guest_name: Vec<String>,
  guest_email: Vec<String>,
}

impl Hotel {
  pub fn new() -> Self {
    // RNG
    let mut rng = thread_rng();
    let res_num_range = Uniform::new(10_000, 100_000);

    // Generates an even total size
    let total_size: u32 = {
      let mut num = rng.gen_range(5..=10);
      if (num & 1) == 1 {
        num += 1;
      }
      num
    };

    // Generates availabity
    let availability: Vec<bool> = (0..total_size)
      .map(|_| rng.gen_bool(0.5))
      .collect();
    // Generates capacity
    let capacity: Vec<u32> = (0..total_size)
      .map(|x| if x < total_size / 2 { 1 } else { 2 })
      .collect();
    // Generates price based on capacity
    let price: Vec<u32> = capacity
      .iter()
      .map(|&x| if x == 1 { 100 } else { 150 })
      .collect();
    // Generates reservation numbers and checks if there are repeats
    let mut used_res_values = HashSet::with_capacity(total_size as usize);
    let reservation_num: Vec<u32> = (0..)
      .map(|_| {
        let random_value = rng.sample(res_num_range);
        if used_res_values.insert(random_value) {
          random_value
        } else {
          0
        }
      })
      .filter(|&x| x != 0)
      .take(total_size as usize)
      .collect();
    // Creates an empty String vector
    let guest_name: Vec<String> = vec![String::new(); total_size as usize];
    // Creates an empty String vector
    let guest_email: Vec<String> = vec![String::new(); total_size as usize];

    // Return Self
    Self {
      total_size,
      availability,
      capacity,
      price,
      reservation_num,
      guest_name,
      guest_email,
    }
  }

  //--------------------------------------------------------------------------------------------

  pub fn reserve_room(
    &mut self,
    room_choice: u32,
    guest_name: &str,
    guest_email: &str
  ) {
    self.availability[(room_choice as usize) - 1] = false;
    self.guest_name[(room_choice as usize) - 1] = guest_name.to_string();
    self.guest_email[(room_choice as usize) - 1] = guest_email.to_string();
  }

  pub fn free_room(&mut self, room_choice: u32) {
    self.availability[(room_choice as usize) - 1] = true;
    self.guest_name[(room_choice as usize) - 1] = String::new();
    self.guest_email[(room_choice as usize) - 1] = String::new();
  }

  //--------------------------------------------------------------------------------------------

  pub fn generate_discount() -> u32 {
    let rand_num = thread_rng().gen_range(1..=10);
    match rand_num {
      1..=2 => 2,
      3..=5 => 1,
      6..=10 => 0,
      _ => panic!(),
    }
  }

  pub fn calculate_price(
    &self,
    room_choice: u32,
    nights: u32,
    discount: u32
  ) -> f32 {
    match discount {
      2 =>
        (self.price[(room_choice as usize) - 1] as f32) * (nights as f32) * 0.8,
      1 =>
        (self.price[(room_choice as usize) - 1] as f32) * (nights as f32) * 0.9,
      0 => (self.price[(room_choice as usize) - 1] as f32) * (nights as f32),
      _ => panic!(),
    }
  }

  //--------------------------------------------------------------------------------------------

  pub fn find_room_num_based(&self, reservation_num: u32) {
    for (i, n) in self.reservation_num.iter().enumerate() {
      if reservation_num == *n {
        println!("Your room number is {}", i + 1);
      }
    }
  }

  pub fn find_room_name_based(&self, guest_name: &str) {
    for (i, n) in self.guest_name.iter().enumerate() {
      if guest_name == *n {
        println!("Your room number is {}", i + 1);
      }
    }
  }

  //--------------------------------------------------------------------------------------------

  pub fn count_free_rooms(&self) -> u32 {
    let mut counter = 0;
    for i in &self.availability {
      if *i {
        counter += 1;
      }
    }
    counter
  }

  pub fn print_free_rooms(&self) {
    println!("Room\tAvailability\tPrice\tReservation number\n");
    for (i, n) in self.availability.iter().enumerate() {
      if *n {
        println!(
          "{}.\tAvailable\t{}\t{}",
          i + 1,
          self.price[i],
          self.reservation_num[i]
        );
      }
    }
  }

  //--------------------------------------------------------------------------------------------

  pub fn print_reservation_num(&self, room_choice: u32) {
    println!(
      "Your reservation number is {}.",
      self.reservation_num[(room_choice as usize) - 1]
    )
  }

  pub fn print_discount(&self, discount: u32) {
    if discount == 2 {
      println!("Congratulations! You have won a 20% discount.")
    } else if discount == 1 {
      println!("Congratulations! You have won a 10% discount.")
    }
  }

  pub fn print_room_capacity(&self, room_choice: u32) {
    if self.capacity[(room_choice as usize) - 1] == 2 {
      println!("You have chosen a two person room.")
    } else {
      println!("You have chosen a one person room.")
    }
  }

  //--------------------------------------------------------------------------------------------

  pub fn check_rand_room_availability(&self) -> u32 {
    let mut rand_num: u32 = 0;
    let mut rng = thread_rng();
    let range = Uniform::new(0, self.total_size);

    for _ in &self.availability {
      rand_num = rng.sample(range);
      if self.availability[rand_num as usize] {
        return rand_num + 1;
      }
    }
    rand_num
  }

  pub fn check_selected_room_availability(&self, room_choice: u32) -> bool {
    if self.availability[(room_choice as usize) - 1] {
      return true;
    }
    false
  }

  pub fn check_cancelabilty(&self, room_choice: u32, guest_name: &str) -> bool {
    if
      !self.availability[(room_choice as usize) - 1] &&
      self.guest_name[(room_choice as usize) - 1] == guest_name
    {
      return true;
    }
    false
  }
}