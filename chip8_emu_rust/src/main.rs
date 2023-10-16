mod chip8;
fn main() {
    let mut cpu = chip8::Chip8Cpu::new();
    cpu.init();
    let _t = 0;
}
