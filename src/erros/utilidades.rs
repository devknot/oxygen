use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Erro {
    Cheio,
}

impl fmt::Display for Erro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Erro::Cheio => "está cheio",
        })
    }
}

impl error::Error for Erro {}

