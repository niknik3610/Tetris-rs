use std::ffi::CString;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::video::GLContext;
use sdl2::video::Window;
use crate::QUAD_SIZE;
use crate::RESOLUTION;
use crate::program::Program;
use crate::shader;
use crate::program;

pub struct GLStructs {
    sdl: Sdl,
    shader_program: Program,
    window: Window,
    _context: GLContext,
    pub event_pump: EventPump,
}

pub struct VideoBuffer {
    pub vertices: Vec<f32>
}
impl VideoBuffer {
    pub fn new() -> VideoBuffer {
        return VideoBuffer { vertices: Vec::new() }
    }
    pub fn clear(&mut self) {
        self.vertices = Vec::new();
    }
    pub fn add_quad(
        &mut self,
        coords: (f32, f32),
        rgb: (u8, u8, u8)
        ) {
        let coords = VideoBuffer::normalize_coords(coords);
        let size = VideoBuffer::normalize_size(QUAD_SIZE); 
        let rgb = VideoBuffer::normalize_rgb(rgb);

        println!("Coords: {:?}", coords);
        println!("Size: {:?}", size);
        println!("RGB: {:?}", rgb);
        self.vertices.extend([
            coords.0, coords.1, 0.0,    rgb.0, rgb.1, rgb.2, // bottom left
            coords.0 + size.0, coords.1, 0.0,     rgb.0, rgb.1, rgb.2, // bottom right
            coords.0, coords.1 + size.1, 0.0,      rgb.0, rgb.1, rgb.2, // top left
         
            coords.0 + size.0, coords.1 + size.1, 0.0,    rgb.0, rgb.1, rgb.2, //top right
            coords.0, coords.1 + size.1, 0.0,      rgb.0, rgb.1, rgb.2, // top left 
            coords.0 + size.0, coords.1, 0.0,     rgb.0, rgb.1, rgb.2, // bottom right
        ]);
    }
    fn normalize_coords(coords: (f32, f32)) -> (f32, f32) {
        return (
            ((coords.0 / RESOLUTION.0 as f32) - 0.5),
            ((coords.1 / RESOLUTION.1 as f32) - 0.5)
        );        
    }
    fn normalize_size(size: (f32, f32)) -> (f32, f32) {
        return (
            (size.0*10.0 / RESOLUTION.0 as f32),
            (size.1*10.0 / RESOLUTION.1 as f32)
        );        
    }
    fn normalize_rgb(rgb: (u8, u8, u8)) -> (f32, f32, f32) {
        return (
            rgb.0 as f32 / 256.0, 
            rgb.1 as f32 / 256.0,
            rgb.2 as f32 / 256.0
            )
    }
}

pub fn init_sdl() -> Result<GLStructs, String> {
    let sdl = sdl2::init().expect("Failed to init SDL");    
    let video_subsystem = sdl.video().expect("Failed to init video");
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Tetris", 900, 700)
        .opengl()
        .build()
        .expect("Failed to start window");
    
    let gl_context = window.gl_create_context().expect("Failed to create Context");
    let _gl = gl::load_with (|f|
        video_subsystem.gl_get_proc_address(f) as *const std::os::raw::c_void
        ); 

    let vert_shader = shader::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
        ).unwrap();

    let frag_shader = shader::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
        ).unwrap();

    let shader_program = program::Program::from_shaders(
        &[vert_shader, frag_shader]
        ).unwrap();
    unsafe { 
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);    
    }
    let event_pump = sdl.event_pump().expect("Failed to init event pump");

    let to_return = GLStructs {
        sdl,
        shader_program,
        window,
        _context: gl_context,
        event_pump,
    }; 

    return Ok(to_return);
}

pub fn bind_video_buffer(video_buffer: VideoBuffer) -> Result<u32, ()> {
    //vertex buffer object
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (video_buffer.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            video_buffer.vertices.as_ptr() as *const gl::types::GLvoid,
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

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null() // offset of the first component
            );
        //layout in vertex shader
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
            );
        gl::EnableVertexAttribArray(1);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    return Ok(vao)
}

pub fn render_buffer(gl_context: &mut GLStructs, vertex_array: u32) -> Result<(), String> { 
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        gl_context.shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vertex_array);
            gl::DrawArrays(
                gl::TRIANGLES,  //mode
                0,  //start index of array
                6   //number of indexes
                );
        }
        gl_context.window.gl_swap_window();
    
    return Ok(());
}

