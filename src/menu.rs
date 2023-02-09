#![allow(unused)]
use anyhow::Result;
use clearscreen::ClearScreen::Cls;
use std::{fmt::Debug, io, str::FromStr};

pub enum Menu {
    Main,
    Side1,
    Side2,
    Side3,
    End,
    Error,
}

impl Menu {
    pub fn get_command() -> Result<char> {
        // Create a buffer and read new input
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input: char = input.trim().parse()?;
        // .expect("Failed to parse user input as char");

        // Clear the screen
        Cls.clear().expect("failed to clear the screen");
        Ok(input)
    }

    pub fn rin<T>(input: &mut T)
    where
        T: FromStr + Debug,
        <T as FromStr>::Err: Debug,
    {
        let mut input_string = String::new();

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read user input");
        *input = input_string
            .trim()
            .parse()
            .expect("Failed to parse user input");
    }

    //--------------------------------------------------------------------------------------------
    // The number in function name indicates the amount of choices for user switching

    // Switch Self without command
    pub fn switch0(&mut self, cond: Self) {
        *self = cond;
        // Clear the screen
        Cls.clear().expect("failed to clear the screen");
    }

    pub fn switch1(&mut self, cond1: Self) {
        let input = match Self::get_command() {
            Ok(x) => x,
            Err(_) => 'x',
        };
        // Read and match input to command
        match input {
            '1' => *self = cond1,
            'l' => *self = Self::End,
            'L' => *self = Self::End,
            _ => *self = Self::Error,
        }
    }

    pub fn switch2(&mut self, cond1: Self, cond2: Self) {
        let input = match Self::get_command() {
            Ok(x) => x,
            Err(_) => 'x',
        };
        // Read and match input to command
        match input {
            '1' => *self = cond1,
            '2' => *self = cond2,
            'l' => *self = Self::End,
            'L' => *self = Self::End,
            _ => *self = Self::Error,
        }
    }

    pub fn switch3(&mut self, cond1: Self, cond2: Self, cond3: Self) {
        let input = match Self::get_command() {
            Ok(x) => x,
            Err(_) => 'x',
        };
        // Read and match input to command
        match input {
            '1' => *self = cond1,
            '2' => *self = cond2,
            '3' => *self = cond3,
            'l' => *self = Self::End,
            'L' => *self = Self::End,
            _ => *self = Self::Error,
        }
    }

    pub fn switch4(&mut self, cond1: Self, cond2: Self, cond3: Self, cond4: Self) {
        let input = match Self::get_command() {
            Ok(x) => x,
            Err(_) => 'x',
        };
        // Read and match input to command
        match input {
            '1' => *self = cond1,
            '2' => *self = cond2,
            '3' => *self = cond3,
            '4' => *self = cond4,
            'l' => *self = Self::End,
            'L' => *self = Self::End,
            _ => *self = Self::Error,
        }
    }
}
