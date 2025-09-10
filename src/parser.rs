use std::fmt::Display;
use std::fmt::Formatter;


pub struct Player {
    pub id: i32, // The player id (assigned by the VM, starting from 1)
    pub name: String,
    pub comment: String,
    pub code: Vec<u8>,
    pub size: u32,
    pub start_address: usize, // Where in the arena the player's code is loaded
}

impl Player {
    pub fn new(
        id: i32,
        name: String,
        comment: String,
        code: Vec<u8>,
        size: u32,
        start_address: usize,
    ) -> Self {
        Self {
            id,
            name,
            comment,
            code,
            size,
            start_address,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
