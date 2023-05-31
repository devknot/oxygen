use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Erro {
    Unknown,
}

impl fmt::Display for Erro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Erro::Unknown    => "erro desconhecido",
        })
    }
}

impl error::Error for Erro {}


