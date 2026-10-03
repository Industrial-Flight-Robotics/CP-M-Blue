pub struct I8080{
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


pub struct Flags
{
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub auxiliary_carry: bool,
}

impl I8080{
    pub fn new() -> Self{
        return I8080 { a: 0x00, b: 0x00, c: 0x00, d: 0x00, e: 0x00, h: 0x00, l: 0x00, pc: 0x0000, sp: 0x0000 };
    }

    pub fn reset(&mut self) {
        self.a = 0x00;
        self.b = 0x00;
        self.c = 0x00;
        self.d = 0x00;
        self.e = 0x00;
        self.h = 0x00;
        self.l = 0x00;
        self.pc = 0x0000;
        self.sp = 0x0000;
    }

    pub fn show_state(&self) {
        println!("\nCPU State:");
        println!("A: {:02X}", self.a);
        println!("B: {:02X}", self.b);
        println!("C: {:02X}", self.c);
        println!("D: {:02X}", self.d);
        println!("E: {:02X}", self.e);
        println!("H: {:02X}", self.h);
        println!("L: {:02X}", self.l);
        println!("PC: {:04X}", self.pc);
        println!("SP: {:04X}", self.sp);
    }
}


impl Flags
{
    pub fn new() -> Flags
    {
        return Flags
        {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
            auxiliary_carry: false,
        };
    }


    pub fn show(&self) {
        println!("\nFlags State:");
        println!("Zero Flag: {}", self.zero);
        println!("Sign Flag: {}", self.sign);
        println!("Parity Flag: {}", self.parity);
        println!("Carry Flag: {}", self.carry);
        println!("Auxiliary Carry Flag: {}", self.auxiliary_carry);
    }
}

