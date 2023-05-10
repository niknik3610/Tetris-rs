use crate::error_handler::Error;

pub enum GLError {
    SdlInit,
    VideoInit,
    WindowInit,
    ContextCreation,
    CompileShader,
    EventPumpInit,
}
impl Error for GLError {
    fn to_string(&self) -> String {
        use GLError::*;
        match self{
            SdlInit => return "Error Initilizing SDL".to_owned(),
            VideoInit => return "Error Initilizing Video".to_owned(),
            WindowInit => return "Error Initilizing Window".to_owned(),
            ContextCreation => return "Error Creating OpenGL context".to_owned(),
            CompileShader => return "Error Compiling Shaders".to_owned(),
            EventPumpInit => return "Error Initlizing EventPump".to_owned()
        }  
    }
}
