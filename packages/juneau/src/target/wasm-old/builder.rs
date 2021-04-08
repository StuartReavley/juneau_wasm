




pub struct Builder {
    buffer:Vec<u8>
}


impl Builder {
    pub fn new() -> Self {
        let buffer = Vec::new();
        Self {buffer}
    }

    pub fn write(&mut self, values:&[u8]) {
        self.buffer.extend(values)
    }
}