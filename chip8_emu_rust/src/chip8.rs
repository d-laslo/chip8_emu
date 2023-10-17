mod fonts;
mod parameters;

pub struct Chip8Cpu {
    pc: u16,
    sp: u16,
    i: u16,
    v: [u8; parameters::REGISTERS_COUNT],
    stack: [u16; parameters::STACK_SIZE],
    memory: [u8; parameters::MEMORY_SIZE],
    screen: [[u8; parameters::SCREEN_HEIGH]; parameters::SCREEN_WIDTH],
}

impl Chip8Cpu{
    pub fn new() -> Self {
        let mut cpu = Chip8Cpu{
            pc: 0, 
            sp: 0, 
            i: 0, 
            v: [0; parameters::REGISTERS_COUNT], 
            stack: [0; parameters::STACK_SIZE], 
            memory: [0; parameters::MEMORY_SIZE],
            screen: [[0; parameters::SCREEN_HEIGH]; parameters::SCREEN_WIDTH]
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
        self.v = [0; parameters::REGISTERS_COUNT];
        self.stack = [0; parameters::STACK_SIZE];
        self.memory = [0; parameters::MEMORY_SIZE];
        self.screen = [[0; parameters::SCREEN_HEIGH]; parameters::SCREEN_WIDTH];
        
        let s = fonts::SMALL.len();
        for i in 0..s {
            self.memory[i] = fonts::SMALL[i];
        }
        for i in 0..fonts::BIG.len() {
            self.memory[i + s] = fonts::BIG[i];
        }
    }
}