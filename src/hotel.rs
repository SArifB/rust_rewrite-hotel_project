#![allow(unused)]
use rand::{thread_rng, Rng};

pub struct Hotel {
    hotel_size: usize,
    availability: Vec<bool>,
    capacity: Vec<usize>,
    price: Vec<usize>,
    reservation_num: Vec<usize>,
    guest_name: Vec<String>,
    guest_email: Vec<String>,
}

impl Hotel {
    pub fn new() -> Self {
        // RNG thread
        let mut rng = thread_rng();

        // Hotel size
        let hotel_size: usize = {
            let mut num = rng.gen_range(5..10);
            if num & 1 == 1 {
                num += 1
            }
            num
        };

        // Rest of the variables
        let availability: Vec<bool> = vec![rng.gen(); hotel_size];
        let capacity: Vec<usize> = vec![if rng.gen() { 1 } else { 2 }; hotel_size];
        let price: Vec<usize> = capacity
            .iter()
            .map(|x| if *x == 1 { 100 } else { 150 })
            .collect();
        let reservation_num: Vec<usize> = (0..hotel_size)
            .map(|_| rng.gen_range(10_000..100_000))
            .collect();
        let guest_name: Vec<String> = vec![String::new(); hotel_size];
        let guest_email: Vec<String> = vec![String::new(); hotel_size];

        // Return Self
        Self {
            hotel_size,
            availability,
            capacity,
            price,
            reservation_num,
            guest_name,
            guest_email,
        }
    }

    //--------------------------------------------------------------------------------------------

    pub fn reserve_room(&mut self, room_choice: usize, guest_name: &str, guest_email: &str) {
        self.availability[room_choice - 1] = false;
        self.guest_name[room_choice - 1] = guest_name.to_string();
        self.guest_email[room_choice - 1] = guest_email.to_string();
    }

    pub fn free_room(&mut self, room_choice: usize) {
        self.availability[room_choice - 1] = true;
        self.guest_name[room_choice - 1] = String::new();
        self.guest_email[room_choice - 1] = String::new();
    }

    //--------------------------------------------------------------------------------------------

    pub fn generate_discount() -> usize {
        // let rand_num = thread_rng().gen_range(1..=10);
        let rand_num = Self::random_value(1, 10);
        match rand_num {
            1..=2 => 2,
            3..=5 => 1,
            6..=10 => 0,
            _ => panic!(),
        }
    }

    pub fn calculate_price(&self, room_choice: usize, nights: usize, discount: usize) -> f64 {
        match discount {
            2 => self.price[room_choice] as f64 * nights as f64 * 0.8,
            1 => self.price[room_choice] as f64 * nights as f64 * 0.9,
            0 => self.price[room_choice] as f64 * nights as f64,
            _ => panic!(),
        }
    }

    //--------------------------------------------------------------------------------------------

    pub fn find_room_num_bsd(&self, reservation_num: usize) {
        for (i, n) in self.reservation_num.iter().enumerate() {
            if reservation_num == *n {
                println!("Your reservation number is {}", i + 1);
            }
        }
    }

    pub fn find_room_name_bsd(&self, guest_name: &str) {
        for (i, n) in self.guest_name.iter().enumerate() {
            if guest_name == *n {
                println!("Your reservation number is {}", i + 1);
            }
        }
    }

    //--------------------------------------------------------------------------------------------

    pub fn count_free_rooms(&self) -> usize {
        let mut counter = 0;
        for i in self.availability.iter() {
            if *i {
                counter += 1;
            }
        }
        counter
    }

    //--------------------------------------------------------------------------------------------

    pub fn print_free_rooms(&self) {
        println!("Room\tAvailability\tPrice\tReservation number\n");
        for (i, n) in self.availability.iter().enumerate() {
            if *n {
                println!(
                    "{}.\tAvailable\t{}\t{}",
                    i + 1,
                    self.price[i],
                    self.reservation_num[i]
                )
            }
        }
    }

    //--------------------------------------------------------------------------------------------

    pub fn print_reservation_num(&self, room_choice: usize) {
        println!(
            "Your reservation number is {}.",
            self.reservation_num[room_choice - 1]
        )
    }

    pub fn print_discount(&self, room_choice: usize, discount: usize) {
        if discount == 2 {
            println!("Congratulations! You have won a 20% discount.")
        } else if discount == 1 {
            println!("Congratulations! You have won a 10% discount.")
        }
    }

    pub fn print_room_capacity(&self, room_choice: usize) {
        if self.capacity[room_choice - 1] == 2 {
            println!("You have chosen a two person room.")
        } else {
            println!("You have chosen a one person room.")
        }
    }

    //--------------------------------------------------------------------------------------------

    pub fn check_rand_room_ava(&self) -> usize {
        let mut rand_num: usize = 0;
        let mut rng = thread_rng();

        for i in self.availability.iter() {
            if *i {
                // rand_num = Self::random_value(0, self.hotel_size as usize - 1);
                rand_num = rng.gen_range(0..self.hotel_size);
                return rand_num + 1;
            }
        }
        rand_num
    }

    pub fn check_ava(&self, room_choice: usize) -> bool {
        if self.availability[room_choice - 1] {
            return true;
        }
        false
    }

    pub fn check_cancel(&self, room_choice: usize, guest_name: &str) -> bool {
        if !self.availability[room_choice - 1] && self.guest_name[room_choice - 1] == guest_name {
            return true;
        }
        false
    }

    //--------------------------------------------------------------------------------------------

    fn random_value(floor: usize, ceil: usize) -> usize {
        // let mut thread_rng = thread_rng();
        let rand_num: usize = thread_rng().gen_range(floor..=ceil);
        rand_num
    }
}
