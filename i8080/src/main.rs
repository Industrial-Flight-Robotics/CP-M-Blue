mod memory;
mod i8080;

use memory::Memory;
use i8080::I8080;

fn main() {
    let mut cpu = i8080::I8080::new();
    let mut flags = i8080::Flags::new();


    cpu.show_state();
    flags.show();
}
