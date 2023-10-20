mod chip8;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

fn draw(canvas: &mut Canvas<Window>, cpu: &chip8::Chip8Cpu)
{
    for y in 0..64  {
        for x in 0..128 {
            if cpu.screen[x][y] == 1 {
                canvas.fill_rect(Rect::new(x as i32 * 8, y as i32 * 8, 8, 8)).unwrap();
            }
        }
    } 
    //canvas.fill_rect(rect)
}

fn handle_key_up( keycode : &Keycode, cpu: &mut chip8::Chip8Cpu)
{
    match keycode {
        Keycode::Num1 => cpu.key[0x1] = true,
        Keycode::Num2 => cpu.key[0x2] = true,
        Keycode::Num3 => cpu.key[0x3] = true,
        Keycode::Num4 => cpu.key[0xC] = true,
        Keycode::Q => cpu.key[0x4] = true,
        Keycode::W => cpu.key[0x5] = true,
        Keycode::E => cpu.key[0x6] = true,
        Keycode::R => cpu.key[0xD] = true,
        Keycode::A => cpu.key[0x7] = true,
        Keycode::S => cpu.key[0x8] = true,
        Keycode::D => cpu.key[0x9] = true,
        Keycode::F => cpu.key[0xE] = true,
        Keycode::Z => cpu.key[0xA] = true,
        Keycode::X => cpu.key[0x0] = true,
        Keycode::C => cpu.key[0xB] = true,
        Keycode::V => cpu.key[0xF] = true,
        _=> print!(""),
    }
}

fn handle_key_down( keycode : &Keycode, cpu: &mut chip8::Chip8Cpu)
{
    match keycode {
        Keycode::Num1 => cpu.key[0x1] = false,
        Keycode::Num2 => cpu.key[0x2] = false,
        Keycode::Num3 => cpu.key[0x3] = false,
        Keycode::Num4 => cpu.key[0xC] = false,
        Keycode::Q => cpu.key[0x4] = false,
        Keycode::W => cpu.key[0x5] = false,
        Keycode::E => cpu.key[0x6] = false,
        Keycode::R => cpu.key[0xD] = false,
        Keycode::A => cpu.key[0x7] = false,
        Keycode::S => cpu.key[0x8] = false,
        Keycode::D => cpu.key[0x9] = false,
        Keycode::F => cpu.key[0xE] = false,
        Keycode::Z => cpu.key[0xA] = false,
        Keycode::X => cpu.key[0x0] = false,
        Keycode::C => cpu.key[0xB] = false,
        Keycode::V => cpu.key[0xF] = false,
        _=> print!(""),
    }
}

fn main() {
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut cpu = chip8::Chip8Cpu::new();
    cpu.load_game("".to_string());


    let window = video_subsystem.window("CHIP_EMU", 1024, 512)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        //canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyUp { keycode , ..} => handle_key_up(&keycode.unwrap(), &mut cpu),
                Event::KeyDown { keycode , ..} => handle_key_down(&keycode.unwrap(), &mut cpu),
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        cpu.execute_opcode();
        draw(&mut canvas, &cpu);
        

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
