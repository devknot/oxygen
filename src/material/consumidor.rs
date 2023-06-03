
use crate::utilidades::energia;
use crate::erros;
use super::consumivel;

pub trait Consumidor {
    type Resultante;
    fn consumir(&mut self) -> Result<Self::Resultante, erros::material::Erro>;
}


