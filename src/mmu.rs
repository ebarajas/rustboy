struct MMU {
    ram: [u8; 8192]
}

impl MMU {
    fn new() -> Self {
        Self { 
            ram: [0; 8192] // Init all RAM to 0
        } 
    }
}