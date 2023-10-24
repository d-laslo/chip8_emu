mod TUIWrapper;
use crate::chip8::parameters::{SCREEN_WIDTH, SCREEN_HEIGH};
use sdl2::pixels::Color;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct SDLWrapper
{
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    canvas:Canvas<Window>,
    scale: u8,
    background_color: (u8, u8, u8),
    object_color: (u8, u8, u8),
}

impl SDLWrapper
{
    pub fn new(scale : Option<u8>, background_color: Option<(u8, u8, u8)>, object_color: Option<(u8, u8, u8)>) -> Self
    {
        if scale == 0 || scale == None { scale = 1}
        if background_color == None { background_color = (255, 255, 255)}
        if object_color == None { object_color = (0, 0, 0)}

        let mut wrapper = SDLWrapper {
            sdl_context: sdl2::init().unwrap(),
            video_subsystem: sdl_context.video().unwrap(),
            window: video_subsystem.window("CHIP_EMU", 1024, 512)
            .position_centered()
            .build()
            .unwrap(),
            canvas : sdl2::init().unwrap()
                .sdl_context
                .video().unwrap()
                .video_subsystem
                .window("CHIP_EMU", SCREEN_WIDTH * scale, SCREEN_HEIGH * scale)
                .position_centered()
                .build().unwrap()
                .window
                .into_canvas()
                .build().unwrap(),
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

impl UIWrapper for SDLWrapper
{
    //pub fn draw()
}