mod fonts;

pub struct Chip8Cpu {
    pc: u16,
    sp: u16,
    i: u16,
    v: [u8; 16],
    stack: [u16; 16],
    memory: [u8; 0xFFF],
}

impl Chip8Cpu{
    pub fn new() -> Self {
        let mut cpu = Chip8Cpu{
            pc: 0, 
            sp: 0, 
            i: 0, 
            v: [0; 16], 
            stack: [0; 16], 
            memory: [0; 0xFFF],
        };
        cpu.init();
        cpu
    }
}

impl Chip8Cpu {
    pub fn init(&mut self) {
        self.pc = 0x200;
        self.sp = 0;
        self.i = 0;
        self.v = [0; 16];
        self.stack = [0; 16];
        self.memory = [0; 0xFFF];
        
        let s = fonts::SMALL.len();
        for i in 0..s {
            self.memory[i] = fonts::SMALL[i];
        }
        for i in 0..fonts::BIG.len() {
            self.memory[i + s] = fonts::BIG[i];
        }
    }
}