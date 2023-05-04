use beryllium::{
    *, 
    init::InitFlags, events::Event
};

fn main() {
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).expect("Failed Setting Major Version");
    sdl.set_gl_context_minor_version(3).expect("Failed Setting Minor Version");
    sdl.set_gl_profile(video::GlProfile::Core).expect("Failed Setting OpenGL to Core");

    let window = sdl.create_gl_window(
            video::CreateWinArgs { 
                title: "Hello World", 
                width: 400, 
                height: 400, 
                allow_high_dpi: true, 
                borderless: false, 
                resizable: false 
            }             
        ).expect("Failed to Create Window");

    'event_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (Event::Quit, _) => break 'event_loop,
                (_, a) => println!("TimeStamp: {a}"),
            }
        }
    }
}
