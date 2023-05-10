use gl::types::GLuint;

use crate::opengl_backend::shader::{Shader, create_empty_cstring};

pub struct Program {
    pub id: GLuint, 
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };
        shaders.iter()
            .for_each(|shader| unsafe{ gl::AttachShader(program_id, shader.id)});
        unsafe { gl::LinkProgram(program_id) }

        let mut success = 1;
        unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success) }

        if success == 0 {
            let mut len = 0;
            unsafe { gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len)}

            let error = create_empty_cstring(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut i8
                    )
            }
            return Err(error.to_string_lossy().into_owned())
        }

        shaders.iter()
            .for_each(|shader| unsafe{ gl::DetachShader(program_id, shader.id)});
        Ok(Program {id: program_id})
    }
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
