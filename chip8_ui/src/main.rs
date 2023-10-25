mod sdl_wrapper;
use log::LevelFilter;
use std::fs::File;
use std::io::Write;
fn main() {
    let target = Box::new(File::create("log.txt").expect("Can't create file"));
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {}",
                record.level(),
                record.args()
            )
        })
        .init();

    let mut wrapper = sdl_wrapper::Wrapper::new(Some(8), None, None);
    wrapper.run("/home/sovun/projects/chip8_emu/games/Tetris.ch8".to_string());
}
