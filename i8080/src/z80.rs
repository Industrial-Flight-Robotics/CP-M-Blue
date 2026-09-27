pub struct Z80{
    pub a: u8,

    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
}


impl Z80{
    pub fn new() -> Self{
        return Z80 { a: 0x00, b: 0x00, c: 0x00, d: 0x00, e: 0x00, h: 0x00, l: 0x00, pc: 0x0000, sp: 0x0000 };
    }
}