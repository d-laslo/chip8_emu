mod fonts;
mod parameters;
use log::{debug, error};
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
pub struct Chip8Cpu {
    pc: u16,
    sp: u16,
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    super_chip_mode: bool,
    stop: bool,
    v: [u8; parameters::REGISTERS_COUNT],
    flags: [u8; parameters::REGISTERS_COUNT],
    stack: [u16; parameters::STACK_SIZE],
    memory: [u8; parameters::MEMORY_SIZE],
    screen: [[u8; parameters::SCREEN_HEIGH]; parameters::SCREEN_WIDTH],
    key: [bool; parameters::KEYS_COUNT],
}

impl Chip8Cpu{
    pub fn new() -> Self {
        let mut cpu = Chip8Cpu{
            pc: 0, 
            sp: 0, 
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            super_chip_mode: false,
            stop: false,
            v: [0; parameters::REGISTERS_COUNT], 
            flags: [0; parameters::REGISTERS_COUNT], 
            stack: [0; parameters::STACK_SIZE], 
            memory: [0; parameters::MEMORY_SIZE],
            screen: [[0; parameters::SCREEN_HEIGH]; parameters::SCREEN_WIDTH],
            key: [false; parameters::KEYS_COUNT],
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
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.super_chip_mode = false;
        self.stop = false;
        self.v = [0; parameters::REGISTERS_COUNT];
        self.flags = [0; parameters::REGISTERS_COUNT];
        self.stack = [0; parameters::STACK_SIZE];
        self.memory = [0; parameters::MEMORY_SIZE];
        self.screen = [[0; parameters::SCREEN_HEIGH]; parameters::SCREEN_WIDTH];
        self.key = [false; parameters::KEYS_COUNT];
        
        let s = fonts::SMALL.len();
        for i in 0..s {
            self.memory[i] = fonts::SMALL[i];
        }
        for i in 0..fonts::BIG.len() {
            self.memory[i + s] = fonts::BIG[i];
        }
    }
}

impl Chip8Cpu
{
    fn execute_opcode(&mut self)
    {
        let opcode : u16 = ((self.memory[self.pc as usize] as u16) << 8) ^ self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        
        match opcode & 0xF000 {
            0x0000 => {
                // 00Cn - SCD nibble
                if (opcode & 0x00F0) == 0xC0 {
                    let n = (opcode & 0x000F) as usize;                    
                    for y in (parameters::SCREEN_HEIGH - 1)..=n {
                        for x in 0..parameters::SCREEN_WIDTH {
                            self.screen[x][y] = self.screen[x][y - n];
                        }
                    }
                    for y in 0..n {
                        for x in 0..parameters::SCREEN_WIDTH {
                            self.screen[x][y] = 0;
                        }
                    }
                    return;
                }
                match opcode & 0x0FFF {
                    // 00E0 - CLS
                    0x00E0 => self.screen = [[0; parameters::SCREEN_HEIGH]; parameters::SCREEN_WIDTH],
                    // 00EE - RET
                    0x00EE => {self.sp -= 1; self.pc = self.stack[self.sp as usize];},
                    // 00FB - SCR
                    0x00FB => {
                        for x in (parameters::SCREEN_WIDTH - 1)..=4 {
                            self.screen[x] = self.screen[x - 4];
                        }
                        for x in 0..4 {
                            self.screen[x] = [0; parameters::SCREEN_HEIGH];
                        }
                    },
                    // 00FC - SCL
                    0x00FC => {
                        for x in 0..(parameters::SCREEN_WIDTH - 4) {
                            self.screen[x] = self.screen[x + 4];
                        }
                        for x in (parameters::SCREEN_WIDTH - 4)..parameters::SCREEN_WIDTH {
                            self.screen[x] = [0; parameters::SCREEN_HEIGH];
                        }
                    },
                    // 00FD - EXIT
                    0x00FD => self.stop = true,
                    // 00FE - LOW
                    0x00FE => self.super_chip_mode = false,
                    // 00FF - HIGH
                    0x00FF => self.super_chip_mode = true,
                    // 0nnn - SYS addr
                    _=> self.pc = opcode & 0x0FFF,
                }
            },
            // 1nnn - JP addr
            0x1000 => self.pc = opcode & 0x0FFF,
            // 2nnn - CALL addr
            0x2000 => {self.sp +=1; self.sp = self.pc; self.pc = opcode & 0x0FFF},
            // 3xkk - SE Vx, byte
            0x3000 => {
                if self.v[(opcode & 0x0F00) as usize >> 0x8] == (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            // 4xkk - SNE Vx, byte
            0x4000 => {
                if self.v[(opcode & 0x0F00) as usize >> 0x8] != (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            // 5xy0 - SE Vx, Vy
            0x5000 => {
                if self.v[(opcode & 0x0F00) as usize >> 0x8] == self.v[(opcode & 0x00F0) as usize >> 0x4]{
                    self.pc += 2;
                }
            }
            // 6xkk - LD Vx, byte
            0x6000 => self.v[(opcode & 0x0F00) as usize >> 0x8] = (opcode & 0x00FF) as u8,
            //7xkk - ADD Vx, byte
            0x7000 => self.v[(opcode & 0x0F00) as usize >> 0x8] += (opcode & 0x00FF) as u8,
            0x8000 => {
                match opcode & 0x000F {
                    // 8xy0 - LD Vx, Vy
                    0x0 => self.v[(opcode & 0x0F00) as usize >> 0x8] = self.v[(opcode & 0x00F0) as usize >> 0x4],
                    // 8xy1 - OR Vx, Vy
                    0x1 => self.v[(opcode & 0x0F00) as usize >> 0x8] |= self.v[(opcode & 0x00F0) as usize >> 0x4],
                    // 8xy2 - AND Vx, Vy
                    0x2 => self.v[(opcode & 0x0F00) as usize >> 0x8] &= self.v[(opcode & 0x00F0) as usize >> 0x4],
                    // 8xy3 - XOR Vx, Vy
                    0x3 => self.v[(opcode & 0x0F00) as usize >> 0x8] ^= self.v[(opcode & 0x00F0) as usize >> 0x4],
                    // 8xy4 - ADD Vx, Vy
                    0x4 => {
                        let res = self.v[(opcode & 0x0F00) as usize >> 0x8] as u16 + self.v[(opcode & 0x00F0) as usize >> 0x4] as u16;
                        if res > 0xFF {self.v[0xF] = 1;} else { self.v[0xF] = 0;}
                        self.v[(opcode & 0x0F00) as usize >> 0x8] = res as u8;
                    }
                    // 8xy5 - SUB Vx, Vy
                    0x5 => {
                        if self.v[(opcode & 0x0F00) as usize >> 0x8] > self.v[(opcode & 0x00F0) as usize >> 0x4] 
                        {self.v[0xF] = 1;} else { self.v[0xF] = 0;}
                        self.v[(opcode & 0x0F00) as usize >> 0x8] -= self.v[(opcode & 0x00F0) as usize >> 0x4];
                    }
                    // 8xy6 - SHR Vx {, Vy}
                    0x6 => {
                        if (self.v[(opcode & 0x0F00) as usize >> 0x8] & 0x1) == 0x1 
                        {self.v[0xF] = 1;} else { self.v[0xF] = 0;}
                        self.v[(opcode & 0x0F00) as usize >> 0x8] >>= 1;
                    }
                    // 8xy7 - SUBN Vx, Vy
                    0x7 => {
                        if self.v[(opcode & 0x0F00) as usize >> 0x8] < self.v[(opcode & 0x00F0) as usize >> 0x4] 
                        {self.v[0xF] = 1;} else { self.v[0xF] = 0;}
                        self.v[(opcode & 0x00F0) as usize >> 0x4] -= self.v[(opcode & 0x0F00) as usize >> 0x8];
                    }
                    // 8xyE - SHL Vx {, Vy}
                    0xE => {
                        if (self.v[(opcode & 0x0F00) as usize >> 0x8] & 0xF) == 0x8 
                        {self.v[0xF] = 1;} else { self.v[0xF] = 0;}
                        self.v[(opcode & 0x0F00) as usize >> 0x8] <<= 1;
                    }
                    _=> error!(""),
                }
            },
            // 9xy0 - SNE Vx, Vy
            0x9000 => if self.v[(opcode & 0x0F00) as usize >> 0x8] != self.v[(opcode & 0x00F0) as usize >> 0x4] {self.pc += 2;},
            // Annn - LD I, addr
            0xA000 => self.i = opcode & 0x0FFF,
            // Bnnn - JP V0, addr
            0xB000 => self.pc = (opcode & 0x0FFF) + self.v[0] as u16,
            // Cxkk - RND Vx, byte
            0xC000 => self.v[(opcode & 0x0F00) as usize >> 8] = rand::thread_rng().gen::<u8>() & (opcode & 0x00FF) as u8,
            // Dxyn - DRW Vx, Vy, nibble
            0xD000 => self.draw_sprite(
                (opcode & 0x0F00) as u8 >> 8, 
                (opcode & 0x00F0) as u8 >> 4, 
                (opcode & 0x000F) as u8),
            0xE000 => {
                match opcode & 0x00FF {
                    // Ex9E - SKP Vx
                    0x9E => if self.key[self.v[(opcode & 0x0F00) as usize >> 8] as usize] {self.pc += 2;},
                    // ExA1 - SKNP Vx
                    0xA1 => if !self.key[self.v[(opcode & 0x0F00) as usize >> 8] as usize] {self.pc += 2;},
                    _=> error!(""),
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    // Fx07 - LD Vx, DT
                    0x07 => self.v[(opcode & 0x0F00) as usize >> 8] = self.delay_timer,
                    // Fx0A - LD Vx, K
                    0x0A => {
                        let mut ind : Option<usize> = None;
                        while ind == None {
                            ind = self.key.iter().position(|&r| r );
                        }
                        self.v[(opcode & 0x0F00) as usize >> 8] = ind.unwrap() as u8;
                    },
                    // Fx15 - LD DT, Vx
                    0x15 => self.delay_timer = self.v[(opcode & 0x0F00) as usize >> 8],
                    // Fx18 - LD ST, Vx
                    0x18 => self.sound_timer = self.v[(opcode & 0x0F00) as usize >> 8],
                    // Fx1E - ADD I, Vx
                    0x1E => self.i += self.v[(opcode & 0x0F00) as usize >> 8] as u16,
                    // Fx29 - LD F, Vx 
                    0x29 => self.i = self.v[(opcode & 0x0F00) as usize >> 8] as u16 * 5,
                    // Fx30 - LD HF, Vx
                    0x30 => self.i = self.v[(opcode & 0x0F00) as usize >> 8] as u16 * 10 + 80,
                    // Fx33 - LD B, Vx
                    0x33 => {
                        let mut t = self.v[(opcode & 0x0F00) as usize >> 8];
                        self.memory[(self.i + 2) as usize] = t % 10; t /= 10;
                        self.memory[(self.i + 1) as usize] = t % 10; t /= 10;
                        self.memory[self.i as usize] = t % 10;
                    }
                    // Fx55 - LD [I], Vx
                    0x55 => {
                        for i in 0..=((opcode & 0x0F00) >> 8) as usize {
                            self.memory[self.i as usize + i] = self.v[i];
                        }
                    }
                    // Fx65 - LD Vx, [I]
                    0x65 => {
                        for i in 0..=((opcode & 0x0F00) >> 8) {
                            self.v[i as usize] = self.memory[self.i as usize + i as usize];
                        }
                    }
                    // Fx75 - LD R, Vx
                    0x75 => self.flags = self.v.clone(),
                    // Fx85 - LD Vx, R
                    0x85 => self.v = self.flags.clone(),
                    _=> error!(""),
                }
            }

            _=> error!(""),
        }

    }
}

impl Chip8Cpu
{
    fn draw_sprite(&mut self, x : u8, y : u8, num : u8)
    {
        error!("TODO: Draw sprite");
    }
}

impl Chip8Cpu
{
    pub fn load_game(path: String)
    {
        // let mut file = File::open("foo.txt")?;
    }
}