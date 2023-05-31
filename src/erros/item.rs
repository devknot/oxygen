use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Erro {
    Nec,
    Espaco,
}

impl fmt::Display for Erro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Erro::Nec    => "não é combustível",
            Erro::Espaco => "não há espaço o suficiente para armazenar",
        })
    }
}

impl error::Error for Erro {}



