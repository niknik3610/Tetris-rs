pub mod gl_error;
pub mod program;
pub mod shader;

use crate::QUAD_SIZE;
use crate::RESOLUTION;
use gl_error::GLError;
use program::Program;

use sdl2::video::GLContext;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use std::ffi::CString;

pub struct GLStructs {
    sdl: Sdl,
    shader_program: Program,
    window: Window,
    _context: GLContext,
    pub event_pump: EventPump,
}

pub struct VideoBuffer {
    pub bg_verts: Vec<f32>,
    pub fg_verts: Vec<f32>,
}
impl VideoBuffer {
    pub fn new() -> VideoBuffer {
        return VideoBuffer {
            bg_verts: Vec::new(),
            fg_verts: Vec::new(),
        };
    }
    pub fn add_quad_fg(&mut self, coords: (f32, f32), rgb: (u8, u8, u8)) {
        let coords = VideoBuffer::normalize_coords(coords);
        let rgb = VideoBuffer::normalize_rgb(rgb);
        let size = VideoBuffer::normalize_size(QUAD_SIZE);
        // println!("Coords: {:?}", coords);
        // println!("Size: {:?}", size);
        // println!("RGB: {:?}", rgb);
        self.fg_verts.extend([
            coords.0,
            coords.1,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // bottom left
            coords.0 + size,
            coords.1,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // bottom right
            coords.0,
            coords.1 + size,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // top left
            coords.0 + size,
            coords.1 + size,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, //top right
            coords.0,
            coords.1 + size,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // top left
            coords.0 + size,
            coords.1,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // bottom right
        ])
    }
    pub fn add_quad_bg(&mut self, coords: (f32, f32), rgb: (u8, u8, u8)) {
        let coords = VideoBuffer::normalize_coords(coords);
        let rgb = VideoBuffer::normalize_rgb(rgb);
        let size = VideoBuffer::normalize_size(QUAD_SIZE);
        // println!("Coords: {:?}", coords);
        // println!("Size: {:?}", size);
        // println!("RGB: {:?}", rgb);
        self.bg_verts.extend([
            coords.0,
            coords.1,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // bottom left
            coords.0 + size,
            coords.1,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // bottom right
            coords.0,
            coords.1 + size,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // top left
            coords.0 + size,
            coords.1 + size,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, //top right
            coords.0,
            coords.1 + size,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // top left
            coords.0 + size,
            coords.1,
            0.0,
            rgb.0,
            rgb.1,
            rgb.2, // bottom right
        ])
    }
    pub fn clear_fg(&mut self) {
        self.fg_verts = Vec::new();
    }
    ///Generally Doesn't need to be used
    pub fn clear_bg(&mut self) {
        self.fg_verts = Vec::new();
    }
    fn normalize_coords(coords: (f32, f32)) -> (f32, f32) {
        return (
            ((coords.0 / RESOLUTION.0 as f32) - 1.0),
            ((coords.1 / RESOLUTION.1 as f32) - 1.0),
        );
    }
    fn normalize_size(size: f32) -> f32 {
        return size / RESOLUTION.0 as f32;
    }
    fn normalize_rgb(rgb: (u8, u8, u8)) -> (f32, f32, f32) {
        return (
            rgb.0 as f32 / 255.0,
            rgb.1 as f32 / 255.0,
            rgb.2 as f32 / 255.0,
        );
    }
}

pub fn init_sdl() -> Result<GLStructs, GLError> {
    let sdl = sdl2::init().map_err(|_| GLError::SdlInit)?;
    let video_subsystem = sdl.video().map_err(|_| GLError::VideoInit)?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Tetris", RESOLUTION.0, RESOLUTION.1)
        .opengl()
        .build()
        .map_err(|_| GLError::WindowInit)?;
    let gl_context = window
        .gl_create_context()
        .map_err(|_| GLError::ContextCreation)?;
    let _gl =
        gl::load_with(|f| video_subsystem.gl_get_proc_address(f) as *const std::os::raw::c_void);

    let vert_shader = shader::Shader::from_vert_source(
        &CString::new(include_str!("opengl_backend/triangle.vert")).unwrap(),
    )
    .map_err(|_| GLError::CompileShader)?;

    let frag_shader = shader::Shader::from_frag_source(
        &CString::new(include_str!("opengl_backend/triangle.frag")).unwrap(),
    )
    .map_err(|_| GLError::CompileShader)?;

    let shader_program = program::Program::from_shaders(&[vert_shader, frag_shader])
        .map_err(|_| GLError::CompileShader)?;
    unsafe {
        gl::Viewport(
            0,
            0,
            (RESOLUTION.0 * 2).try_into().unwrap(),
            (RESOLUTION.1 * 2).try_into().unwrap(),
        );
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    let event_pump = sdl.event_pump().map_err(|_| GLError::EventPumpInit)?;
    let to_return = GLStructs {
        sdl,
        shader_program,
        window,
        _context: gl_context,
        event_pump,
    };

    return Ok(to_return);
}

pub fn bind_video_buffer(video_buffer: &VideoBuffer) -> Result<u32, ()> {
    //vertex buffer object
    let video_buffer = [video_buffer.bg_verts.clone(), video_buffer.fg_verts.clone()].concat();

    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (video_buffer.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            video_buffer.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
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
            std::ptr::null(), // offset of the first component
        );
        //layout in vertex shader
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    return Ok(vao);
}

pub fn render_buffer(
    gl_context: &mut GLStructs,
    vertex_array: u32,
    amount_of_quads: i32,
) -> Result<(), String> {
    let amount_of_quads = amount_of_quads * 6;
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    gl_context.shader_program.set_used();
    unsafe {
        gl::BindVertexArray(vertex_array);
        gl::DrawArrays(
            gl::TRIANGLES,   //mode
            0,               //start index of array
            amount_of_quads, //number of indexes
        );
    }
    gl_context.window.gl_swap_window();

    return Ok(());
}
