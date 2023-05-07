mod shader;
mod program;
use sdl2::{event::Event as SdlEvent, video::gl_attr};
use std::ffi::{CStr, CString};
use gl::types::GLuint;

fn main() {
    let sdl = sdl2::init().expect("Failed to init SDL");    
    let video_subsystem = sdl.video().expect("Failed to init video");
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);

    let window = video_subsystem
        .window("Tetris", 960, 590)
        .opengl()
        .build()
        .expect("Failed to start window");
    let gl_context = window.gl_create_context().expect("Failed to create Context");
    let gl = gl::load_with (|f|
        video_subsystem.gl_get_proc_address(f) as *const std::os::raw::c_void
        );
    
    unsafe { 
        gl::Viewport(0, 0, 960, 590);
        gl::ClearColor(0.0, 0.5, 1.0, 1.0);
    }

    let vert_shader = shader::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
        ).unwrap();

    let frag_shader = shader::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
        ).unwrap();

    let shader_program = program::Program::from_shaders(
        &[vert_shader, frag_shader]
        ).unwrap();
    shader_program.set_used();

    let vertices = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    //vertex buffer object
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
            );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //vertex array object
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        //layout in vertex shader
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
            );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut event_pump = sdl.event_pump().expect("Failed to init event pump");
    'run_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                SdlEvent::Quit {..} => break 'run_loop,
                _ => {}
            }
        }

        shader_program.set_used();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,  //mode
                0,  //start index of array
                3   //number of indexes
                )
        }
        window.gl_swap_window();
    }
}
