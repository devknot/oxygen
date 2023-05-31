use std::fmt;

type Joule = u64;

#[derive(Debug)]
pub enum Erro {
    Combustivel,
}

impl fmt::Display for Erro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "problema: {}", match self {
            Erro::Combustivel => "sem material à consumir",
            _ => "desconhecido",
        })
    }
}

impl std::error::Error for Erro {}

pub trait Usina<Material> {
    fn consumir(&mut self) -> Result<Joule, Erro>;
    fn Abastecer(&mut self, cb: Material) -> Result<Joule, Erro>;
}

