use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Erro {
    Nec, // não é um combustível
    Espaco,
}

impl fmt::Display for Erro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Erro::Nec    => "não é um combustível",
            Erro::Espaco => "não há espaço o suficiente para armazenar",
        })
    }
}

impl error::Error for Erro {}

