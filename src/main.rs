use sdl2::event::Event as SdlEvent;
fn main() {
    let sdl = sdl2::init().expect("Failed to init SDL");    
    let video_subsystem = sdl.video().expect("Failed to init video");
    let window = video_subsystem.window(
        "Tetris", 
        800, 
        400 
        )
        .build()
        .expect("Failed to start window");
    let mut event_pump = sdl.event_pump().expect("Failed to init event pump");
    'run_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                SdlEvent::Quit {..} => break 'run_loop,
                _ => {}
            }
        }
    }
}
