use std::fmt::Debug;

pub trait Error {
    fn to_string(&self) -> String;
}

impl Debug for dyn Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        eprintln!("{}", self.to_string());
        return Ok(());
    }
}
