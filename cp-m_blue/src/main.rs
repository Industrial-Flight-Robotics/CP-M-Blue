pub struct Cpu {
     pub pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self { pc: 0 }
    }

    pub fn step(&mut self){
        self.pc += 1;
    }
}

fn main() {

    let mut cpu = Cpu::new();

    for _ in 0..10 {
        cpu.step();
        println!("PC: {}", cpu.pc);
    }
    
}
