use crate::{CStr, CString, GLuint};

pub struct Shader {
    pub id: GLuint,
}
impl Shader {
    pub fn from_source(source: &CStr, kind: GLuint) -> Result<Shader, String> {
        let id = shader_from_string(source, kind)?;
        Ok(Shader { id })
    }
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        let id = shader_from_string(source, gl::VERTEX_SHADER)?;
        return Ok(Shader { id });
    }
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        let id = shader_from_string(source, gl::FRAGMENT_SHADER)?;
        return Ok(Shader { id });
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_string(source: &CStr, kind: GLuint) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    let mut success = 1;
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut err_len = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut err_len);
        }
        let buffer = create_empty_cstring(err_len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                err_len + 1,
                std::ptr::null_mut(),
                buffer.as_ptr() as *mut gl::types::GLchar,
            );
        }
        return Err(buffer.to_string_lossy().into_owned());
    }
    return Ok(id);
}

pub fn create_empty_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
