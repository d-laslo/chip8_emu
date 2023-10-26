mod sdl_wrapper;
// use log::LevelFilter;
// use std::fs::File;
// use std::io::Write;

fn main() {
    // let target = Box::new(File::create("log.log").expect("Can't create file"));
    // env_logger::Builder::new()
    //     .target(env_logger::Target::Pipe(target))
    //     .filter(None, LevelFilter::Debug)
    //     .format(|buf, record| {
    //         writeln!(
    //             buf,
    //             "[{}] {}",
    //             record.level(),
    //             record.args()
    //         )
    //     })
    //     .init();
    
    let mut wrapper = sdl_wrapper::Wrapper::new(Some(16), None, None);
    let mut stop = false;
    while !stop
    {
        main_msg();
        let enter = get_enter();
        if enter == "e\n"
        {
            stop = true;
        }
        else if enter == "lg\n" {
            select_game_msg();
            let enter = get_enter();
            let mut gm: String = "".to_string();
            if enter == "0\n" { gm = "test_opcode.ch8".to_string();}
            else if enter == "1\n" { gm = "15puzzle.ch8".to_string();}
            else if enter == "2\n" { gm = "tetris.ch8".to_string();}
            else if enter == "3\n" { gm = "blinky.ch8".to_string();}
            else { println!("--> unexpected game\n");}
            if gm.len() > 0 {
                wrapper.run("../games/".to_string() + &gm );
            }
        }
        else { println!("--> unexpected command\n"); }
    }
    
}

fn get_enter() -> String
{
    let mut enter: String = String::new();
    std::io::stdin().read_line(&mut enter).unwrap();
    enter
}

fn main_msg()
{
    println!("--> Enter the command");
    println!("--> e: exit");
    println!("--> lg: load game");
}

fn select_game_msg()
{
    println!("--> Enter the game");
    println!("--> 0: test opcodes");
    println!("--> 1: 15puzzle");
    println!("--> 2: tetris");
    println!("--> 3: blinky");
}