pub struct Player {
    pub id: i32, // The player id (assigned by the VM, starting from 1)
    pub name: String,
    pub comment: String,
    pub code: Vec<u8>,
    pub size: u32,
    pub start_address: usize, // Where in the arena the player's code is loaded
}


impl Player {
    pub fn new(){
        todo!()
    }
}