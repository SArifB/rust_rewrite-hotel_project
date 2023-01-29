#![allow(unused)]
use core::fmt::Debug;
use std::io;

pub enum Menu {
    Main,
    Side1,
    Side2,
    Side3,
    End,
}

impl Menu {
    fn rin_char() -> char {
        let mut input = String::new();
        // Create and read new input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input: char = input
            .trim()
            .parse()
            .expect("Failed to parse user input as char");
        input
    }

    fn clear_screen() {
        // Clear the screen
        clearscreen::ClearScreen::Cls
            .clear()
            .expect("failed to clear the screen");
    }

    pub fn switch(&mut self, condition: Menu) {
        // Switch Menu without arguments
        *self = condition;

        // Clear the screen
        Self::clear_screen();
    }

    pub fn switch1(&mut self, condition1: Menu) {
        // Create and read new input
        let input = Self::rin_char();

        // Match input to predisposed commands
        match input {
            '1' => *self = condition1,
            'l' => *self = Menu::End,
            'L' => *self = Menu::End,
            _ => *self = Menu::End,
        }

        // Clear the screen
        Self::clear_screen();
    }

    pub fn switch2(&mut self, condition1: Menu, condition2: Menu) {
        // Create and read new input
        let input = Self::rin_char();

        // Match input to predisposed commands
        match input {
            '1' => *self = condition1,
            '2' => *self = condition2,
            'l' => *self = Menu::End,
            'L' => *self = Menu::End,
            _ => *self = Menu::End,
        }

        // Clear the screen
        Self::clear_screen();
    }

    pub fn switch3(&mut self, condition1: Menu, condition2: Menu, condition3: Menu) {
        // Create and read new input
        let input = Self::rin_char();

        // Match input to predisposed commands
        match input {
            '1' => *self = condition1,
            '2' => *self = condition2,
            '3' => *self = condition3,
            'l' => *self = Menu::End,
            'L' => *self = Menu::End,
            _ => *self = Menu::End,
        }

        // Clear the screen
        Self::clear_screen();
    }

    pub fn switch4(&mut self, cond1: Menu, cond2: Menu, cond3: Menu, cond4: Menu) {
        // Create and read new input
        let input = Self::rin_char();

        // Match input to predisposed commands
        match input {
            '1' => *self = cond1,
            '2' => *self = cond2,
            '3' => *self = cond3,
            '4' => *self = cond4,
            'l' => *self = Menu::End,
            'L' => *self = Menu::End,
            _ => *self = Menu::End,
        }

        // Clear the screen
        Self::clear_screen();
    }
}
