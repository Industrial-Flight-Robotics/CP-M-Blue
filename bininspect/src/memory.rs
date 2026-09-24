pub struct Memory
{
    data: [u8; 65536],
}

impl Memory
{
    pub fn new() -> Memory
    {
        return Memory
        {
            data: [0; 65536],
        };
    }

    pub fn read(&self, address: u16) -> u8
    {
        return self.data[address as usize];
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        self.data[address as usize] = value;
    }

    pub fn load(&mut self, start_address: u16, program: &[u8])
    {
        let start = start_address as usize;

        for i in 0..program.len()
        {
            self.data[start + i] = program[i];
        }
    }
}