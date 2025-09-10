use std::fmt::Display;
use std::fmt::Formatter;
use vm::*;

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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // make a table
        let mut table = Table::new();
        // add headers id, name, comment
        table.add_header("id");
        table.add_header("name");
        table.add_header("comment");
        // add rows
        table.add_row(&vec![
            self.id.to_string(),
            self.name.clone(),
            self.comment.clone(),
        ]);
        println!("{table}");
        // make a table
        // add headers code , size start_address,
        // add rows
        Ok(())
    }
}
