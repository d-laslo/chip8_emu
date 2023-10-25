use chip8_cpu::chip8::Chip8Cpu;
use chip8_cpu::chip8::parameters::{SCREEN_WIDTH, SCREEN_HEIGH};
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use chrono::Local;



pub struct Wrapper
{
    cpu: Chip8Cpu,
    sdl_context: Sdl,
    //video_subsystem: VideoSubsystem,
    canvas:Canvas<Window>,
    scale: u32,
    background_color: (u8, u8, u8),
    object_color: (u8, u8, u8),
}

impl Wrapper
{
    pub fn new(mut scale : Option<u8>, mut background_color: Option<(u8, u8, u8)>, mut object_color: Option<(u8, u8, u8)>) -> Self
    {
        if scale.unwrap() == 0 || scale == None { scale = Some(1);}
        if background_color == None { background_color = Some((255, 255, 255));}
        if object_color == None { object_color = Some((0, 0, 0));}

        let sdl_context = sdl2::init().unwrap();
        //let video_subsystem = sdl_context.video().unwrap();
        // let window = video_subsystem.window("CHIP_EMU", (SCREEN_WIDTH * scale.unwrap() as usize) as u32, (SCREEN_HEIGH * scale.unwrap() as usize) as u32)
        //     .position_centered()
        //     .build()
        //     .unwrap();
        let canvas = sdl_context
            .video()
            .unwrap()
            .window("CHIP_EMU", (SCREEN_WIDTH * scale.unwrap() as usize) as u32, (SCREEN_HEIGH * scale.unwrap() as usize) as u32)
            .position_centered()
            .build()
            .unwrap()
            .into_canvas()
            .build().unwrap();

        let mut wrapper = Wrapper {
            cpu: Chip8Cpu::new(),
            sdl_context: sdl_context,
            //video_subsystem: video_subsystem,
            canvas : canvas,
            scale : scale.unwrap() as u32,
            background_color : background_color.unwrap(),
            object_color : object_color.unwrap(),
        };

        wrapper.canvas.set_draw_color(Color::RGB(
            wrapper.background_color.0, 
            wrapper.background_color.1,
            wrapper.background_color.2));
        wrapper.canvas.clear();
        wrapper.canvas.present();
        wrapper
    }
}

impl Wrapper
{
    fn get_timestamp(&self) -> i64
    {
        Local::now().timestamp_millis()
    }

    pub fn run(&mut self, path: String)
    {
        self.cpu.init();
        self.cpu.load_game(path);
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut last_tick = self.get_timestamp();

        let mut cycles_per_second : i32;
        let mut opcode_count = 0;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'running,
                    Event::KeyUp { keycode , ..} => self.handle_key_up(&keycode.unwrap()),
                    Event::KeyDown { keycode , ..} => self.handle_key_down(&keycode.unwrap()),
                    _ => {}
                }
            }
            if self.cpu.stop {break 'running;}

            if self.cpu.super_chip_mode { cycles_per_second = 30;}
            else { cycles_per_second = 10; }

            if opcode_count < cycles_per_second
            {
                self.cpu.execute_opcode();
                opcode_count += 1;
            }

            if self.get_timestamp() - last_tick >= 1000/60 {
                self.cpu.decrease_timers();
                last_tick = self.get_timestamp();

                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                self.draw();
                self.canvas.present();

                opcode_count = 0;
            }
        }
    }
}

impl Wrapper
{
    fn draw(&mut self)
    {
        for y in 0..64  {
            for x in 0..128 {
                if self.cpu.screen[x][y] == 1 {
                    self.canvas.set_draw_color(Color::RGB(
                        self.object_color.0, 
                        self.object_color.1,
                        self.object_color.2));
                    self.canvas.fill_rect(Rect::new(x as i32 * self.scale as i32, y as i32 * self.scale as i32, self.scale, self.scale), ).unwrap();
                }
                else {
                    self.canvas.set_draw_color(Color::RGB(
                        self.background_color.0, 
                        self.background_color.1,
                        self.background_color.2));
                    self.canvas.fill_rect(Rect::new(x as i32 * self.scale as i32, y as i32 * self.scale as i32, self.scale, self.scale), ).unwrap();
                }
            }
        } 
    }
}

impl Wrapper
{
    fn handle_key_down(&mut self, keycode : &Keycode)
    {
        match keycode {
            Keycode::Num1 => self.cpu.key[0x1] = true,
            Keycode::Num2 => self.cpu.key[0x2] = true,
            Keycode::Num3 => self.cpu.key[0x3] = true,
            Keycode::Num4 => self.cpu.key[0xC] = true,
            Keycode::Q => self.cpu.key[0x4] = true,
            Keycode::W => self.cpu.key[0x5] = true,
            Keycode::E => self.cpu.key[0x6] = true,
            Keycode::R => self.cpu.key[0xD] = true,
            Keycode::A => self.cpu.key[0x7] = true,
            Keycode::S => self.cpu.key[0x8] = true,
            Keycode::D => self.cpu.key[0x9] = true,
            Keycode::F => self.cpu.key[0xE] = true,
            Keycode::Z => self.cpu.key[0xA] = true,
            Keycode::X => self.cpu.key[0x0] = true,
            Keycode::C => self.cpu.key[0xB] = true,
            Keycode::V => self.cpu.key[0xF] = true,
            _=> print!(""),
        }
    }

    fn handle_key_up(&mut self, keycode : &Keycode)
    {
        match keycode {
            Keycode::Num1 => self.cpu.key[0x1] = false,
            Keycode::Num2 => self.cpu.key[0x2] = false,
            Keycode::Num3 => self.cpu.key[0x3] = false,
            Keycode::Num4 => self.cpu.key[0xC] = false,
            Keycode::Q => self.cpu.key[0x4] = false,
            Keycode::W => self.cpu.key[0x5] = false,
            Keycode::E => self.cpu.key[0x6] = false,
            Keycode::R => self.cpu.key[0xD] = false,
            Keycode::A => self.cpu.key[0x7] = false,
            Keycode::S => self.cpu.key[0x8] = false,
            Keycode::D => self.cpu.key[0x9] = false,
            Keycode::F => self.cpu.key[0xE] = false,
            Keycode::Z => self.cpu.key[0xA] = false,
            Keycode::X => self.cpu.key[0x0] = false,
            Keycode::C => self.cpu.key[0xB] = false,
            Keycode::V => self.cpu.key[0xF] = false,
            _=> print!(""),
        }
    }
}